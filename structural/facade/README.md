> `The Facade Pattern` provides a unified interface to a set of interfaces in a subsystem. Facade defines a higher level interface that makes the subsystem easier to use.

A facade not only simplifies an interface, it decouples a client from a subsystem of components. Facades and adapters may wrap multiple classes, but a facade's intent is to simplify, while an adapter's is to convert the interface to something different.

# Use case
In Chapter 7 of HFDP Book, people may find it hard to watch the movie for a home theater system, because you need to perform the few tasks (e.g, turn on the popcorn popper, start the popper popping, dim the lights, put the screen on, turn the projector on ...). We can use `Facade Pattern` to hide the complexities and expose a few simple methods such as `watchMovie()`.

In our code, for simplicity, we only implement the *radio* related logic in the facade.