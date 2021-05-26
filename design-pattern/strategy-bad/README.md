> `The Strategy Pattern` defines a family of algorithms, encapsulates each one, and makes them interchangeable. Strategy lets the algorithm vary independently from clients that use it.

This module `strategy-bad` illustrates a bad design, which totally borrows the codes in `Java`. But remember, `Rust` is not a traditional OO language. Particularly, it does not support inheritance. Therefore, we have to repeat the **same** `get/set` implementations in every struct that implements `Duck`.

To implement `strategy pattern` in `Rust`, we could pass the behavior trait as a parameter, instead of change its behavior via `setter`.