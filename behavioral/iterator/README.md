> `The Iterator Pattern` provides a way to access the elements of an aggregate object sequentially without exposing its underlying representation.

The key point in Rust is to distinguish `iter()` and `into_iter()`. See more at [Effectively Using Iterators In Rust](https://hermanradtke.com/2015/06/22/effectively-using-iterators-in-rust.html).

**tl;dr**: 

- Use the `iter()` function if you want to iterate over the values by *reference*.
- Use the `into_iter()` function when you want to *move*, instead of *borrow*.
- By default, the `for` loop will apply the `into_iter()` function to the collection.

Starting from Rust 1.53.0, the behavior of iteration for `array` has changed. See more at [IntoIterator for arrays](https://blog.rust-lang.org/2021/06/17/Rust-1.53.0.html). Now we can iterate over arrays by value:

```
for i in [1, 2, 3] {

}
```
Previously, this was only possible by reference, using `&[1, 2, 3]` or `[1, 2, 3].iter()`.

# Use case
In Chapter 9 of HFDP Book, Diner and Pancake House are merged. The waitress is faced with the problem to iterate the two different menu representations. Additionally, we should also hide the internal data structures from the waitress. In other words, we would like to encapsulate the iteration.

Also, this design also adheres to `The Single Responsibility Principle`. The Menu should only take care of its own business (managing menu items), instead of take on more responsibilities (like iteration).


---
There are many ways to implement `iteration pattern` in Rust. In our code, we simply take the leverage of its built-in `iter()` support:

```
fn iter(&self) -> std::slice::Iter<'_, MenuItem> {
    self.menu_items[..self.number].iter()
}
```
And you may find more approaches in [Returning Rust Iterators](https://depth-first.com/articles/2020/06/22/returning-rust-iterators/) and [How to implement Iterator and IntoIterator for a simple struct?](https://stackoverflow.com/questions/30218886)