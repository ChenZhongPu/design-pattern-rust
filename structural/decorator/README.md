> ` The Decorator Pattern` attaches additional responsibilities to an object dynamically. Decorators provide a flexible alternative to subclassing for extending functionality.

## Use Case

In Chapter 1 of `HFDP Book`, we would like to add condiments to coffees in a more flexible way.

The `decorator pattern` is often useful for adhering to the *Single Responsibility Principle*. The key point is that: `decorator` (`CondimentDecorator`) is a `component` (`Beverage`), and it also has a `component`. In our code, both are traits:

```rust
trait CondimentDecorator: Beverage
```

## Discussion
As for this coffee example, some people would argue that *why not a list of condiments*. See more at [Using Lists Instead of Decorator Pattern?](https://stackoverflow.com/questions/43565475/).

To put it simply, the decorator pattern has an important precondition: **we cannot change the code** while we would like to add new functionalities to it in the runtime.

> The Open-Closed Principle: Classes should be open
for extension, but closed for modification.

In other words, we assume that there was a `Beverage` interface and several concrete coffees. By following the Open-Close Principle, *we cannot change the code*, then how can we add the new functionality (i.e., condiment)? Aha! Decorator Pattern saves your day!

Of course, the *inheritance* is another way to extend functionalities, and it also adheres to the Open-Close Principle. However, compared to Decorator Pattern, using inheritance would be a disaster for code maintenance.