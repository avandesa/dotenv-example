#[macro_use]
extern crate dotenv_codegen;

// use dotenv_codegen::dotenv;

fn main() {
    // NOTE: the macro doesn't put the variables in the environment, it only parses `/.env` and finds the requested variable
    dbg!(dotenv!("FOO"));

    // Uncommenting the below line will cause `cargo build` to fail
    // dbg!(dotenv!("BAZ"));

    // Even though we called the macro, `FOO` is still not defined in the environment
    println!("`FOO` before dotenv: {:?}", std::env::var("FOO"));

    dotenv::dotenv().expect("couldn't load .env file");

    println!("`FOO` after dotenv: {:?}", std::env::var("FOO"));
}
