> `The Observer Pattern` defines a one-to-many dependency between objects so that when one object changes state, all of its dependents are notified and updated automatically.

# Use Case
In the Chapter 2 of `HFDP Book`, we would like to get notifications from `WeatherData`.

# Observer 1
For the ease of explanation, we only consider the `temperature` metric. For more complex metrics, it is better to create a `WeatherEvent` to encapsulate it.

The code of [observer.rs](observer/src/observer.rs) is very easy to understand. As for `Subject`, like the `Duck` in *Strategy Pattern*, it is not necessarily an interface in Java or a trait in Rust. In our code, we consider `WeatherData` as the `Subject` directly, and refactoring is also simple:

```rust
trait Subject {
    fn register_observer(xxx);
    fn remove_observer(xxx);
    fn notify_observers(xxx);
}
```
and then `impl Subject for WeatherData`. Note that if `remove_observer()` is not required, the implementation is much easier.

A small trick for most beginners is how to maintain a collection of observers in a subject. If we still use the trait object `Box<dyn Observer>`, we may find it is impossible to achieve `remove` operation since [Vec](https://doc.rust-lang.org/std/vec/struct.Vec.html)'s `remove` method requires an index. 

(note, [lpxxn](https://github.com/lpxxn/rust-design-pattern/blob/master/behavioral/observer.rs) did not use trait object, so it only supports one specific type of `Observer`.)

```rust
struct WeatherData {
    observers: Vec<Box<dyn Observer>>,
}
```
So, if we would like to remove an item in `Vec`, we must know its position. Hence, one solution is to assign an ID for each observer,

```rust
observers.remove(observers.iter().position(|x| x.getID() == needle.getID() )).expect("needle not found");
```
See more at [Remove an element from a vector](https://stackoverflow.com/questions/26243025/remove-an-element-from-a-vector).

Another solution is to let `Observer` implements `PartialEq`, and then we can apply comparison via `==` directly. But for `trait objects`, it is very complicated. A third way is to use a `Map` (say `HashMap`) in which a unique id is a key.

Furthermore, `Box` would move the object, so every time we would like to register or remove an observer, we would create a new observer. To avoid such cost, we could replace it with `& dyn Observer` which will borrow the object, but a simpler solution is to use `Vec<Rc<dyn Observer>>` instead. And in our code, we use the `& dyn Observer`.

See more at [Lifetime elision](https://doc.rust-lang.org/reference/lifetime-elision.html).

By the way, when we only borrow the object, a unique id is not necessary, because we can compare them through pointer addresses. The complete code can be found at [observer1x](observer1x).

```rust
pub fn remove_observer(&mut self, observer: &'a dyn Observer) {
    if let Some(p) = self.observers.iter().position(|&x| x as *const _ == observer as *const _) {
        self.observers.swap_remove(p);
    }
}
```

## Problem: Does it REALLY update()?
Many tutorials in terms of `Observer Pattern` only simply print some information. But, in reality, it should *really* update something. So, we expect

```rust 
pub trait Observer {
    fn update(&mut self, tmp: f64);
}
```
so, inside `update()` method, we are able to change its state (e.g., temperature). But it is not a trivial work in Rust. We mentioned that 

> but a simpler solution is to use `Vec<Rc<dyn Observer>>` instead.

Can `Rc` solve this problem? No! [Via immutable references, Rc<T> allows you to share data between multiple parts of your program for reading only.](https://doc.rust-lang.org/book/ch15-04-rc.html).

# Observer 2
To address the problem in `Observer 1`, we have to use *Interior mutability*. See more at [RefCell<T> and the Interior Mutability Pattern](https://doc.rust-lang.org/book/ch15-05-interior-mutability.html).

In fact, `Box<RefCell<T>>` will make code cleaner, but it always moves. And we can use `Rc<RefCell<T>>`, and refer to [Rc<RefCell<Dog>> to Rc<RefCell<dyn AnimalT>>](https://users.rust-lang.org/t/rc-refcell-dog-to-rc-refcell-dyn-animalt/29511).

# Observer 3
Still, we have an important question to be solved in both `Observer 1` and `Observer 2`: **should the observer also know the subject**? We don't want to address the notorious **cycle references** issue.

Here, we implement a simple solution by `Weak` references (see more at [How can I implement the observer pattern in Rust?](https://stackoverflow.com/questions/37572734)). The following is how to remove an observer:

```rust
pub fn remove_observer(&mut self, observer: &Rc<dyn Observer>) {
    if let Some(p) = self.observers.iter().position(|x| match x.upgrade() {
        Some(x_ptr) => Rc::ptr_eq(&x_ptr, observer),
        None => false,
    }) {
        self.observers.swap_remove(p);
    }
}
```