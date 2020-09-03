use std::{
    collections::HashMap,
    thread,
    time::{Duration, Instant},
    cmp::Eq,
    hash::Hash,
};
use rand::Rng;

struct Cacher<A, B, F>
    where F: Fn(A) -> B
{
    calc: F,
    map: HashMap<A, B>,
}

impl<A, B, F> Cacher<A, B, F>
    where F: Fn(A) -> B,
          A: Clone + Copy + Eq + Hash,
          B: Copy
{
    pub fn new(calc: F) -> Cacher<A, B, F> {
        Cacher {
            calc,
            map: HashMap::new(),
        }
    }

    pub fn call(&mut self, x: A) -> B {
        match self.map.get(&x) {
            Some(&y) => y,
            None => {
                let calculated = (self.calc)(x);
                match self.map.insert(x, calculated) {
                    Some(y) => y,
                    None => calculated,
                }
            }
        }
    }
}

fn memoize<A, B, F>(f: F) -> impl FnMut(A) -> B
    where F: Fn(A) -> B,
          A: Clone + Copy + Eq + Hash,
          B: Copy
{
    let mut cacher = Cacher::new(f);

    move |x| cacher.call(x)
}

fn measure_time<A, B, F>(mut f: F, x: A) -> f32
    where F: FnMut(A)-> B
{
    let now = Instant::now();
    f(x);
    now.elapsed().as_secs_f32()
}

#[test]
fn test_memoize() {
    let f = |x: i32| -> bool {
        thread::sleep(Duration::from_secs(1));
        x < 1 << 10
    };
    let mut f = memoize(f);

    assert_eq!(f(2), true);
    assert!(measure_time(f, 2) < 1f32);
}

#[test]
fn test_memoize_rand() {
    let f = |_| -> f64 { rand::random::<f64>() };
    let mut f = memoize(f);

    assert_eq!(f(()), f(()));
}

#[test]
fn test_memoize_rand_with_seed() {
    let mut rng = rand::thread_rng();
    let f = move |_| {
        thread::sleep(Duration::from_secs(1));
        rng.gen::<f64>()
    };

    assert!(measure_time(f, ()) > 0.9f32);
    assert!(measure_time(f, ()) > 0.9f32);
}
