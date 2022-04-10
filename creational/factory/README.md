> ` The Factory Method Pattern` defines an interface for creating an object, but lets subclasses decide which class to instantiate. Factory Method lets a class defer instantiation to subclasses.

See more at [wikipedia](https://en.wikipedia.org/wiki/Factory_method_pattern).

# Use Case
In the Chapter 4 of `HFDP Book`, we would like to instantiate concrete `Pizza` objects in a *factory*.

# Simple Pizza Factory
**The Simple Factory isn’t actually a Design Pattern**; it’s more of a programming idiom. But it is commonly used. Some developers do mistake this idiom for the *Factory Pattern*. This idiom is known as `static factory method`.

In `HFDP Book`, the pizza store holds a reference to the factory, but it is not required. 

# Factory Method

> `The Factory Method Pattern` defines an interface for creating an object, abut let subclass decide which class to instantiate. Factory Method lets a class defer instantiation to subclass.

Firstly, we merge `store` and `factory` here; in other words, `store` is essentially a `factory` which can produce products. In the light of `dependency inversion principle`, both `store` and `pizza` are abstract (*trait* in Rust).

A factory method handles object creation and encapsulates it in a subclass:

```rust
trait PizzaStore {
    fn create_pizza(&self, t: &str) -> Box<dyn Pizza>;
}
```

There are two main roles here: `Factory` (or `Creator`) and `Product`. And generally, there is also a method in `Factory` using the `Product`; in other words, the application code (i.e., the use of a product) is also found in `Factory`. In our this example, it is the `order_pizza()` method.

# Abstract Factory
> `The Abstract Factory Pattern` provides an interface for creating families of related or dependent objects without specifying their concrete classes.

As for `factory`, in `factory method pattern`, it mainly relies on **inheritance** (using trait implementation in Rust); in `abstract factory`, it mainly relies on **composition**. To be specific, there is an abstract *factory*, which will be composited by client code.

In general, In factory method pattern, we have abstract factory and product, but there is only one kind of product (pizza in the use case). However, we notice that to prepare a pizza, we need several kinds of ingredients. So, in abstract factory pattern, Product = Ingredient 1 + Ingredient 2 + .... Here, ingredients are related or dependent objects.

For a concrete pizza, it contains several ingredients which should be `null` when instantiating the `pizza`. In Rust, we use `Option` to donate `null`.

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
Note that how those ingredients are assembled has nothing to do the pattern itself (i.e., how the store creates a pizza). Here `Pizza` has a reference to `factory`, and this is the logic for clients.