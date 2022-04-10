> `The Singleton Pattern` ensures a class has only one instance, and provides a global point of access of it.

Although the `Singleton` is the simplest in terms of its class diagram, it is difficult to get it right due to multi-threads. In Java, a quick fix-up is to add the `synchronized` keywords. But we can also note that **after the first time through, synchronization is totally unneeded overhead**.

```java
public static synchronized Singleton getInstance() {
    if (uniqueInstance == null) {
        uniqueInstance = new Singleton();
    }
    return uniqueInstance;
}
```
An alternative is to *move to an early crated instance*:

```java
private static final Singleton uniqueInstance = new Singleton();

public static Singleton getInstance() {
    return uniqueInstance;
}
```

We can also use *double-checked locking*:

```java
private volatile static Singleton uniqueInstance;

public static Singleton getInstance() {
    if (uniqueInstance == null) {
        synchronized (Singleton.class) {
            if (uniqueInstance == null) {
                uniqueInstance = new Singleton();
            }
        }
    }
}
```

**HFDP Book**, as well as **Effective Java**, also provide a better approach:

> a single-element
enum type is often the best way to implement a singleton.

Back at Rust, the good thing is that it provides better concurrency guarantee, and the bad thing is that `static` filed is not allowed in structures. It is highly recommended to use [lazy_static](https://crates.io/crates/lazy_static) if you want to use `static`. As for the `Singleton` pattern, we also adopt the `enum` type as the implementation.

```rust
pub enum Singleton {
    INSTANCE,
}
```

What's more, we can also attach data to the `enum` if necessary.

```rust
pub enum Singleton {
    INSTANCE(String),
}
```