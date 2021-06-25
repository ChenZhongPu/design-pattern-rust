> `The Composite Pattern` allows you to compose objects into tree structures to represent part-whole hierarchies. Composite lets clients treat individual objects and compositions of objects uniformly.

Using a composite structure, we can apply the same operations over both composites and individual objects. In other words, in most cases we can *ignore* the differences compositions of objects and individual objects.

Note that, `Composite Pattern` does not adhere to the Single Responsibility Principle, and trades it for *transparency*. This is a classic case of tradeoff.

# Use case
In Chapter 9 of HFDP Book, we want to add a dessert submenu, and such structural bears resemblance to `node-leaf` in a tree.