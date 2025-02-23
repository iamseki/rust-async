# Studying Async in Rust

## scraper.rs

- Rust has no async built in runtime, this is in purpose, so different use cases has different performance expectations.

- This example demonstrate that we can't turn the main function into async without any third party library, so we just use a runtime in the main function to execute our async code with some concurrency with race function.

- We also learn that rust translate this code in to the Future Trait for us, doing all the compiling check that we are use to it.

## listener-block.rs

- We deep dive in how comunicate between futures by using channels with some concurrency in mind again with join a fair method to await for futures to complete.

- Also with emphasis that the futureu is lazy, it will executed only when `await` is called.

- This program is a listener that never exits waiting for messages through a unbounded mpsc channel in rx.recv().await, given control back to the runtime do another thing until other message arrives, but the receiver never ends, could be useful for some use casese (pulsar consumer maybe).

## listener.rs

- Now the listener exiting because we use async move, to move ownership to inside of the async block so at the end the tx channel is gone, and when this happens the mpsc channel just closed and when this happend recv() receives None, endind the while let Some(value) = rx.recv().await loop

- Still limited to join method to run the futures concurrently

## yield.rs

- Show how to give control back to the async runtime in cases that we have a CPU bound synchronous blocking code. Using yield between function calls:
```rs
            slow("b", 75);
            trpl::yield_now().await;
            slow("b", 10);
            trpl::yield_now().await;
```
- Watcht out with this manual yield, it might not be goot to performance, needs benchmarking, because its not free
- It might be a good idea to manual yield if you expect concurrency code but it seem a more sequential one
