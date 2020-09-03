/*
 * 1. Identity function
 */

fn id<T>(x: T) -> T { x }

/*
 * 2. Composition function
 */ 

fn comp<'a, A, B, C, F, G>(f: F, g: G) -> impl Fn(A) -> C + 'a
    where F: Fn(A) -> B + 'a,
          G: Fn(B) -> C + 'a,
{
    move |x| g(f(x))
}

/*
 * 3. Tests for composition function
 */

#[test]
fn test_comp_id() {
    {
        let f: fn(i32) -> i32 = |x| x + 2;
        let g = comp(id, f);
        let h = comp(f, id);

        assert_eq!(g(2), 4);
        assert_eq!(h(2), 4);
    }

    {
        let f: fn(bool) -> &'static str = |x| if x { "true" } else { "false" };
        let g = comp(id, f);

        assert_eq!(g(true), "true");
        assert_eq!(g(false), "false");
    }
}
