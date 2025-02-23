# Studying Async in Rust

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