fn f(x: i32) -> i32 { x + 1 }
fn g(x: i32) -> i32 { x * 2 }
fn h(x: i32) -> i32 { x * x }

fn compose<A, B, C, F, G>(f: F, g: G) -> impl Fn(A) -> C
where
    F: Fn(A) -> B,
    G: Fn(B) -> C,
{
    move |x| g(f(x))
}

fn id<A>(x: A) -> A { x }

fn main() {
    let x = 3;

    // 結合性
    let h_g_f = compose(compose(f, g), h);
    let h_g_f2 = compose(f, compose(g, h));
    println!("{}", h_g_f(x));   // 64
    println!("{}", h_g_f2(x));  // 64

    // 恒等射
    let f_id = compose(f, id);
    let id_f = compose(id, f);
    println!("{}", f_id(5));    // 6
    println!("{}", id_f(5));    // 6
    println!("{}", f(5));       // 6
}
