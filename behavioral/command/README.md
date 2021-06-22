> `The Command Pattern` encapsulates a request as an object, thereby letting you parameterize other objects with different requests, queue or log requests, and support undoable operations.

![command](command.jpg)

# Use Case
In the Chapter 6 of `HFDP Book`, we would like to decouple the remote and devices; and we able to plugged different devices to remote's slots and control *on*, *off* by pressing buttons.

# Command 1
In a simple form, imagine there is only one slot on the remote control. For simplicity, we suppose `Command` will not update the *receiver*, and `receiver` will be `moved`.

```rust
struct SimpleRemoteControl {
    slot: Box<dyn Command>,
}
```

Note that in Java, we could use `@FunctionalInterface` to simplify code by lambda expressions (i.e., without implementing `Command` explicitly).

```java
Light light = new Light();
remote.setCommand(light::on);
```

# Command 2
