> `The Strategy Pattern` defines a family of algorithms, encapsulates each one, and makes them interchangeable. Strategy lets the algorithm vary independently from clients that use it.

This module `strategy-bad` illustrates a bad design, which totally borrows the codes in `Java`. But remember, `Rust` is not a traditional OO language. Particularly, it does not support inheritance. Therefore, we have to repeat the **same** `get/set` implementations in every struct that implements `Duck`.

To implement `strategy pattern` in `Rust`, we could pass the behavior trait as a parameter, instead of change its behavior via `setter`.

# Step 1: common behavior
In fact, to understand `strategy pattern`, different ducks are not important here. Given a behavior (or algorithm), we provide different implementations, and clients can choose one of them at runtime, and clients even are able to write its own implementation.

Since all algorithms share a common behavior, we use `trait` to express it. Take `FlyBehavior` for an example,

```rust
trait FlyBehavior {
    fn fly(&self);
}
```
Different implementations of `FlyBehavior` share the common behavior `fly()`.

# Step 2: single Duck
If there is only one kind of `Duck`, file duck2.rs still shows how to design a strategy pattern via composition. Since a duck can use different kinds of FlyBehavior, we should use Box<dyn FlyBehavior>.

# Step 3: multi Ducks
