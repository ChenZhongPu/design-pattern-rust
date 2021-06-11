> `The Singleton Pattern` ensures a class has only one instance, and provides a global point of access of it.

Although the `Singleton` is the simplest in terms of its class diagram, it is difficult to get it right due to multi-threads. In Java, a quick fix-up is to add the `synchronized` keywords. But we can also note that **after the first time through, synchronization is totally unneeded overhead**.

Back at Rust, the good thing is that it provides better concurrency guarantee, and the bad thing is that `static` filed is not allowed in structures. It is highly recommended to use [lazy_static](https://crates.io/crates/lazy_static) to implement `Singleton` in Rust.