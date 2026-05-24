fn f(x: i32) -> i32 {
    x * 2
}

fn g(x: i32) -> i32 {
    x + 1
}

fn compose<A, B, C, F, G>(f: F, g: G) -> impl Fn(A) -> C
where
    F: Fn(A) -> B,
    G: Fn(B) -> C,
{
    move |x| g(f(x))
}

fn main() {
    let g_after_f = compose(f, g);

    println!("{}", g_after_f(3));   // 7
    println!("{}", g_after_f(10));  // 21
}
