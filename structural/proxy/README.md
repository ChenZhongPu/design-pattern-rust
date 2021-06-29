> `The Proxy Pattern` provides a surrogate or placeholder for another object to control access to it.

In Chapter 11 of HFDP Book, extensive Java specific implementations are covered. And our code is mainly based on [wikipedia](https://en.wikipedia.org/wiki/Proxy_pattern). The key point: proxy object and real object implement the same trait.

Use the `Proxy Pattern` to create a representative object that controls access to another object, which may be remote, expensive to create, or in need of securing.

Sometimes `Proxy` and `Decorator` look very similar, but their purposes are different: a decorator adds behavior to a class, while a proxy controls access to it.