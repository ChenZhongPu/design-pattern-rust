> `The Template Method Pattern` defines the skeleton of an algorithm in a method, deferring some steps to subclasses. Template Method lets subclasses redefine certain steps of an algorithm without changing the algorithms' structure.

In short, it defines the steps of an algorithm and allows subclasses to provide the implementation for one or more steps.

# Use case
In Chapter 8 of HFDP Book, we recognize that Tea and Coffee recipes (*boil, brew, pour, add condiments*) are essentially the same, although some of the steps require different implementations. So we will generalize the recipe (i.e., `template method`) and place it in the base class. 


`Template Method` is very common in practise. For example, the `sort()` method relies on `compareTo()` method in Java. And in Rust, we can either use derived trait or lambda. See more at [Sorting Vectors](https://rust-lang-nursery.github.io/rust-cookbook/algorithms/sorting.html).

---

# An Q&A from HFDP Book

>**Q**: This implementation of sorting actually seems more like the Strategy Pattern than the Template Method Pattern. Why do we consider it Template Method?

>**A**: You're probably thinking that because the Strategy Pattern uses object composition. You're right in a way: we're using Arrays object to sort our array, so that's similar to Strategy. But remember, in Strategy, the class that you compose with implements the entire algorithm. The algorithm that Arrays implements for `sort()` is incomplete; it needs a class to fill in the missing `compareTo()` method. So, in that way, it's more like Template Method.