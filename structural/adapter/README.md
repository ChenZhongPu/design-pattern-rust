> `The Adapter Pattern` converts the interface of a class into another interface that clients expect. Adapter lets classes work together that couldn't otherwise because of incompatible interfaces.

# Use case
In Chapter 7 of HFDP Book, we would like to make `Turkey` looks like a `Duck`, so we crate a `TurkeyAdapter` which implements `Duck` and is composed with `Turkey`.

Note that there are actually two kinds of adapters: *object* adapters and *class* adapters. In our code, we use the `object` adapters.

A summary of three similar patterns:

| Pattern | Intent |
|:-------------:| -----:|
| Decorator | Doesn't alter the interface, but adds responsibility |
| Adapter      |   Converts one interface to another |
| Facade      |    Makes an interface simpler |