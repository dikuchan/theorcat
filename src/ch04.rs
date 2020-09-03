/*
    0. Kleisli category for the writer monad
 */

type Writer<A> = (A, String);

fn comp_writer<A, B, C, F, G>(m1: F, m2: G) -> impl Fn(A) -> Writer<C>
    where F: Fn(A) -> Writer<B>,
          G: Fn(B) -> Writer<C>
{
    move |x| {
        let (y, s1) = m1(x);
        let (z, s2) = m2(y);
        (z, format!("{}{}", s1, s2))
    }
}

fn id_writer<A>(x: A) -> Writer<A> { (x, String::new()) }

/*
    1. Kleisli category for partial functions
 */

fn comp_partial<A, B, C, F, G>(m1: F, m2: G) -> impl Fn(A) -> Option<C>
    where F: Fn(A) -> Option<B>,
          G: Fn(B) -> Option<C>
{
    move |x| {
        if let Some(y) = m1(x) {
            if let Some(z) = m2(y) {
                return Some(z);
            }
        }
        None
    }
}

fn id_partial<A>(x: A) -> Option<A> { Some(x) }

fn safe_root(x: f64) -> Option<f64> {
    if x >= 0f64 { Some(x.sqrt()) } else { None }
}

/*
    2. Safe reciprocal implementation
 */

fn safe_reciprocal(x: f64) -> Option<f64> {
    if x.abs() - 1e-3 >= 0f64 { Some(1f64 / x) } else { None }
}

/*
    3. Safe root reciprocal implementation
 */

fn safe_root_reciprocal(x: f64) -> Option<f64> {
    comp_partial(safe_root, safe_reciprocal)(x)
}

#[test]
fn test_partial_functions() {
    assert_eq!(10f64, safe_root_reciprocal(0.01).unwrap());
    assert_eq!(None, safe_root_reciprocal(1e-9));
}
