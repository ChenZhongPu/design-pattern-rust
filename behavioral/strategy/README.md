>  `The Strategy Pattern` defines a family of algorithms, encapsulates each one, and makes them interchangeable. Strategy lets the algorithm vary independently from clients that use it.

# Use Case
In the Chapter 1 of `HFDP Book`, we would like to change the behaviors of a `duck` *at runtime*. And we can also think of a set of behaviors as a family of algorithms.

# Strategy 1
In the `simple SimUDuck` example, the `Duck` itself is an abstract class, and lots of other types of ducks inherit from the Duck class. But in fact, the strategy pattern *does not* require such inheritance. So the minimal UML class diagram (adapted from [wikipedia](https://en.wikipedia.org/wiki/Strategy_pattern)) is illustrated in the following:

![strategy](uml.png)

In Rust, we use trait ([fly_behavior.rs](strategy-1/src/fly_behavior.rs) and [quack_behavior.rs](strategy-1/src/fly_behavior.rs)) to describe such behaviors.

As for composition relationship, it is a bit difference between Rust and other OO languages (e.g., Java and C++). Many beginners may think it is fine to use traits here directly, but unluckily, [*trait is not a valid type in Rust*][1]. Instead, it is possible to use `trait bound`. For example,

```rust
struct Duck<T: FlyBehavior> {
    fly_behavior: T,
}
```
But in this case, one duck can only have one specific `fly_behavior`, and we are unable to change it.

```rust
let mut duck = Duck { fly_behavior: FlyNoWay };
```
So, we must [use `trait objects` that allow for values of different types][2].

Now, we refactor the [duck.rs](strategy-1/src/duck.rs).

```rust
struct Duck {
    fly_behavior: Box<dyn FlyBehavior>,
}
```
(We can also use `& dyn FlyBehavior` type and add an explicit lifetime.)

So far so good, we provide a drive program in [main.rs](strategy-1/src/main.rs). Now, this duck can change its behavior at runtime.

But, if we try to consider inheritances like `HFDP Book`, we will find that *too many* repeated codes are required. To be specific, in every derived duck, the `set_fly_behavior` is the same, and this is because (1) struct inheritance is not supported in Rust, so we have to refactor `Duck` to a trait; (2) trait cannot have fields.

It is possible to achieve repetition-free by macro, but it would add extra complexity. 

# Strategy 2
Our improvement for the problems of `Strategy 1` is quite straightforward. We do not have to let `Duck` refer to a behavior in its fields. Instead, we move it to the parameters of `perform_fly`.

```rust
fn perform_fly<T: FlyBehavior>(&self, fly: &T) {
    fly.fly();
}
```
In this way, any new duck will have a concise implementation. BTW, like `HFDP Book`, we also provide a `display` method for specific implementations, and a `swim` method as a default implementation.

[1]: <https://www.ncameron.org/blog/dyn-trait-and-impl-trait-in-rust> "dyn Trait and impl Trait in Rust"
[2]: <https://doc.rust-lang.org/book/ch17-02-trait-objects.html> "Rust Book"