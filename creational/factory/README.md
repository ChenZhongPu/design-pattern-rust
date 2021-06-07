> ` The Factory Method Pattern` defines an interface for creating an object, but lets subclasses decide which class to instantiate. Factory Method lets a class defer instantiation to subclasses.

See more at [wikipedia](https://en.wikipedia.org/wiki/Factory_method_pattern).

# Use Case
In the Chapter 4 of `HFDP Book`, we would like to instantiate concrete `Pizza` objects in a *factory*.

# Simple Pizza Factory
**The Simple Factory isn’t actually a Design Pattern**; it’s more of a programming idiom. But it is commonly used. Some developers do mistake this idiom for the Factory Pattern.

# Factory Method
Firstly, we merge `store` and `factory` here; in other words, `store` is essentially a `factory` which can produce products. In the light of `dependency inversion principle`, both `store` and `pizza` are abstract (trait in Rust).

Note that the method of `factory` is usually a static one (associate function in Rust), and the refactor is simple:

```rust
trait PizzaStore {
    fn create_pizza(t: &str) -> Box<dyn Pizza>;
```

# Abstract Factory
> `The Abstract Factory Pattern` provides an interface for creating families of related or dependent objects without specifying their concrete classes.

As for `factory`, in `factory method pattern`, it mainly relies on **inheritance** (using trait implementation in Rust); in `abstract factory`, it mainly relies on **composition**. To be specific, there is an abstract *factory*, which will be composited by client code.

In general, In factory method pattern, we have abstract factory and product, but there is only one kind of product (pizza in the use case). However, we notice that to prepare a pizza, we need several different kinds of ingredients. So, in abstract factory pattern, Product = Ingredient 1 + Ingredient 2 + .... Here, ingredients are related or dependent objects.


For a concrete pizza, it contains several ingredients which should be `null` when instantiating the `pizza`. In Rust, we `Option` to donate `null`.

```rust 
pub struct VeggiePizza {
    name: String,
    dough: Option<Box<dyn Dough>>,
    sauce: Option<Box<dyn Sauce>>,
    cheese: Option<Box<dyn Cheese>>,
    veggies: Option<Vec<Box<dyn Veggies>>>,
    factory: Box<dyn PizzaIngredientFactory>,
}
```