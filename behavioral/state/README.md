> `The State Pattern` allows an object to alter its behavior when its internal state changes. The object will appear to change its class.

In fact, `State Pattern` has the same class diagram with `Strategy Pattern`, but the two patterns differ in their *intent*. 

With the State Pattern, we have a set of behaviors encapsulated in state objects; at any time the context is delegating to one of those states. Over time, the current state changes across the set of state objects to reflect the internal state of the context, so the context's behavior changes over time as well. The client usually knowns very little, if anything, about the state objects.

With Strategy, the client usually specifies the strategy object that the context is composed with. Now, while the pattern provides the flexibility to change the strategy object at runtime, often there is a strategy object that is most appropriate for a context object.

- Think of the `Strategy Pattern` as a flexible alternative to subclassing (by composing with a different type).
- Thank of the `State Pattern` as an alternative to putting lots of conditionals in your context.

# Use case
In Chapter 10 of HFDP Book, the Gumball has several states (e.g., no quarter, sold out), and different states can be shifted to each other under certain conditions.


---
In contrast to garbage-collected languages like Java, *self-referential* structures are not permissible within Rust's ownership model. In other words, it is **not** trivial to adapt the following Java snippet into Rust.

```java
class Machine {
    State state;
    Foo(State state) { this.state = state; }
}

class State {
    Machine machine;
    State(Machine machine) { this.machine = machine; }
}
```

See more at [Why can't I store a value and a reference to that value in the same struct?
](https://stackoverflow.com/questions/32300132). 

In our code, we mimic the implementation in [Rust Book: Implementing an Object-Oriented Design Pattern](https://doc.rust-lang.org/book/ch17-03-oo-design-patterns.html). The key is that we change the composition to the parameter:

```rust
fn dispense<'a>(self: Box<Self>, machine: &'a mut GumballMachine) -> Box<dyn State>;
```

BTW, [Rust Book: Implementing an Object-Oriented Design Pattern](https://doc.rust-lang.org/book/ch17-03-oo-design-patterns.html) also provides an alternative solution: *Encoding States and Behavior as Types*.