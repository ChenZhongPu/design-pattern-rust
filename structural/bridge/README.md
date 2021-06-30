> `The Bridge Pattern` is to decouple an abstraction from its implementation so that the two can vary independently.

To be specific, use the Bridge Pattern to vary not only your implementations, but also your abstractions. The followings are quoted from [Bridge](https://refactoring.guru/design-patterns/bridge):

When talking about real applications, the abstraction can be represented by a graphic user interface (GUI), and the implementation could be the underlying operating system code (API) which the GUI layer calls in response to user interactions. Generally speaking, you can extend such an app in two independent directions:

- Have several GUIS (for instance, tailored for regular customers or admins).
- Support several APIs (under Windows, MacOS, and Linux).

# Use case
In Appendix of HFDP Book, we regard `RemoteControl` as abstraction class hierarchy; and regard `TV` as implementation class hierarchy. The relationship between the two is referred to as the `bridge`.

Again, in Rust, it is a little awkward to express `has-a` relationship in `trait`, so one solution is to add a getter method instead.