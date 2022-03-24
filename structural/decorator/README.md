> ` The Decorator Pattern` attaches additional responsibilities to an object dynamically. Decorators provide a flexible alternative to subclassing for extending functionality.

## Use Case

In Chapter 1 of `HFDP Book`, we would like to add condiments to coffees in a more flexible way.

The `decorator pattern` is often useful for adhering to the *Single Responsibility Principle*. The key point is that: `decorator` (`CondimentDecorator`) is a `component` (`Beverage`), and it also has a `component`. In our code, both are traits:

```rust
trait CondimentDecorator: Beverage
```