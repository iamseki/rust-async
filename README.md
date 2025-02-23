# Studying Async in Rust

This is my study and practice of Chapters 16, 17 and 21 from [The Rust Programming Language book](https://rust-book.cs.brown.edu/ch16-00-concurrency.html), focusing on concurrency, async programming, and building a multithreaded web server. It highlights the lessons learned through reading and hands-on practice.

- Each of the following sections points to a binary that you can run using cargo, i.e: `cargo run --bin listener`.


## `scraper.rs`

- Rust does not have a built-in async runtime by  design. This allows different runtimes to optimize for different performance trade-offs based on use cases.

- This example demonstrates that we can't make the `main` function `async` without using a third-party runtime. Instead, we use a runtime in `main` to execute our async code concurrently with the `race` function.

- We also learn that Rust automatically translates async functions into the `Future` trait, performing all the necessary compile-time checks.

## `listener-block.rs`

- We explore how to communicate between async tasks using channels while maintaining concurrency. We use `join`, which fairly awaits multiple futures to complete.

- Emphasizes that futures are **lazy**, they only execute when `.await` is called.

- This program implements a listener that continuously waits for messages via an **unbounded MPSC channel** (`rx.recv().await`). While waiting, it gives control back to the runtime, allowing other tasks to execute until a new message arrives. The receiver never exists, making it useful for ***long-running services like a Pulsar consumer***.

## `listener.rs`

- Unlike `listener-block.rs`, this version exits when the `tx` channel is dropped.

- By using `async move`, we transfer ownership inside the async block. When `tx` is dropped, the **MPSC channel** automatically closes. At that point, `recv().await` returns _None_, ending the `while let Some(msg) = rx.recv().await` loop.

## `yield.rs`

- Demonstrates how to give control back to the async runtime when running CPU-bound synchronous code by manually yielding between function calls:
```rs
            slow("b", 75);
            trpl::yield_now().await;
            slow("b", 10);
            trpl::yield_now().await;
```

- Be cautious with manual yielding, it may negatively impact performance. Benchmarking is required since yielding is not free.

- Manual yielding can be useful in scenarios where you expect concurrency execution but your code behaves more sequentially. However, it should be used only when necessary.

## `timeout.rs`

- More examples on how to use small blocks of futures to build a timeout function that takes an arbitrary `Future`.
- It demonstrates that as long as we adhere to the `Future` contract, we can build our own async abstractions.

## `streams.rs`

- Streams can be used as an asynchronous way to retrive a bounded or unbounded list of items. This is useful for processing data from a network source and similar scenarios.

- We can combine iterators/stream methods like `take`, `throttle`, `timeout`, and others.

- I don't fully understand the usage of `pin!` yet:

```rust
fn main() {
    trpl::run(async {
        let messages = get_messages().timeout(Duration::from_millis(200));
        let intervals = get_intervals()
            .map(|count| format!("Interval: {count}"))
            .throttle(Duration::from_millis(100))
            .timeout(Duration::from_secs(10));

        let merged = messages.merge(intervals).take(20);
        let mut stream = pin!(merged);

        while let Some(result) = stream.next().await {
            match result {
                Ok(msg) => println!("{msg}"),
                Err(reason) => eprintln!("Problem: {reason:?}"),
            }
        }
    });
}
```

- This also demonstrates that infinite loops are quite common in async Rust. Many programs need to keep running indefinitely. **This does not block anything else as long as there is at least one `await` point in each iteration of the loop**.