#![feature(proc_macro_hygiene, stmt_expr_attributes, async_await)]

use std::io;
use futures::{ future, executor };
use prefix_await::aawait;


async fn foo(input: &str) -> &str {
    #[aawait] future::ready(input)
}

async fn foo2(input: &str) -> io::Result<&str> {
    let n = #[aawait(?)] future::ok::<_, io::Error>(input);
    Ok(n)
}

fn main() -> io::Result<()> {
    let hello = executor::block_on(foo("hello"));
    let world = executor::block_on(foo2("world!"))?;

    println!("{} {}", hello, world);

    Ok(())
}
