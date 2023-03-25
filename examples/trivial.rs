use memoize::memoize;

#[memoize]
fn hello(arg: String, arg2: usize) -> bool {
    println!("{} => {}", arg, arg2);
    arg.len() % 2 == arg2
}

#[memoize]
async fn hello2(arg: String) -> bool {
    return true
}

fn main() {
    // `hello` is only called once here.
    assert!(!hello("World".to_string(), 0));
    assert!(!hello("World".to_string(), 0));
    // Sometimes one might need the original function.
    assert!(!memoized_original_hello("World".to_string(), 0));
}
