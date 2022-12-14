# Using tracing with `tracing-subscriber`

Using a fun bitcoin mining analogy. A bitcoin mining facility has 10 mining rigs, each running in their own thread.
All these mining rigs fail one after the other, but in a last effort before dying, they send their bitcoins to the
main process through an mpsc channel.

None of this is real, but this is to test the [tracing crate](https://docs.rs/tracing/latest/tracing/)
in an asynchronous environment, to check if the context can follow separate threads, which occurs a lot in Sōzu.