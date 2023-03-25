use memoize::memoize;

#[memoize]
fn hello(arg: String, arg2: usize) -> bool {
    println!("{} => {}", arg, arg2);
    arg.len() % 2 == arg2
}

#[memoize]
async fn hello2(arg: String) -> bool {
    println!("hello 2 called");
    return true
}

fn main() {
    // `hello` is only called once here.
    assert!(!hello("World".to_string(), 0));
    assert!(!hello("World".to_string(), 0));
    // Sometimes one might need the original function.
    assert!(!memoized_original_hello("World".to_string(), 0));

    let run_twice = || async {
        futures::join!(hello2("".to_string()), hello2("".to_string()))
    };
    // async fns should work too.
    assert!(futures::executor::block_on(run_twice()) == (true, true));
    assert!(futures::executor::block_on(hello2("".to_string())));
    assert!(futures::executor::block_on(hello2("".to_string())));
}
