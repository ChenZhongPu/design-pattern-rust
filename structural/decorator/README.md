> ` The Decorator Pattern` attaches additional responsibilities to an object dynamically. Decorators provide a flexible alternative to subclassing for extending functionality.


The `decorator pattern` is often useful for adhering to the *Single Responsibility Principle*. The key point is that: `decorator` (`CondimentDecorator`) is a `component` (`Beverage`), and it also has a `component`. In our code, both are traits:

```rust
trait CondimentDecorator: Beverage
```