use http::http_request;
fn main() {
    println!("Hello, world!");
    let f = |x: i32| x;
    test_fn(f, 10);
    println!("{}", test_static("10"));
    let mut m = 10;
    match m {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        ref mut n @ 4..=10 => println!("four to ten {}", n),
        12 | 11 => println!("eleven"),
        _ => println!("other"),
    }
}

fn test_fn<F>(f: F, i: i32)
where
    F: Fn(i32) -> i32 + 'static,
{
    println!("{}", f(i));
}

fn test_static<T>(i: T) -> T {
    i
}
