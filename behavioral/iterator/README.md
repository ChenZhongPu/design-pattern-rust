> `The Iterator Pattern` provides a way to access the elements of an aggregate object sequentially without exposing its underlying representation.

Compared with Java, Rust provides better and powerful `iterator` support. For example, we cannot iterate over arrays directly in Java, but iterating over arrays is possible in Rust through `slice`.

The key point in Rust is to distinguish `iter()` and `into_iter()`. See more at [Effectively Using Iterators In Rust](https://hermanradtke.com/2015/06/22/effectively-using-iterators-in-rust.html).

**tl;dr**: 

- Use the `iter()` function if you want to iterate over the values by *reference*.
- Use the `into_iter()` function when you want to *move*, instead of *borrow*.
- By default, the `for` loop will apply the `into_iter()` function to the collection.

Starting from Rust 1.53.0, the behavior of iteration for `array` has changed.