> `The Command Pattern` encapsulates a request as an object, thereby letting you parameterize other objects with different requests, queue or log requests, and support undoable operations.

![command](command.jpg)

# Use Case
In the Chapter 6 of `HFDP Book`, we would like to decouple the remote and devices; and we are able to plugged different devices to remote's slots and control *on*, *off* by pressing buttons.

Four terms always associated with the command pattern are command (e.g., `LightOnCommand`), receiver (e.g., `Light`), invoker (e.g., `SimpleRemoteControl`) and client. 

# Command 1
In a simple form, imagine there is only one slot on the remote control. For simplicity, we suppose `Command` will not update the *receiver*, and `receiver` will be `moved`. Similar to what we did in `Observer Pattern`, we can use `Rc<RefCell<dyn T>>` to make it possible, and we adopt such design in `Command 2`.

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
In this case, the remote control have multiple slots, and in our code, we set it to be `4`. Since `Box<dyn Command>` does not implement `Clone` trait, `vec![Box::new(NoCommand); 4]` is not allowed.

# Command Undo
Implementing `undo` is very easy in `Command Pattern`, and the only difference is to remember the last state; we encapsulate *undo* as a command. Again, the assignment is not allowed:
```rust
// Cannot move
self.undo_command = self.on_commands[slot];
```

There are several solutions. One is to make `Box<dyn Command>` clone-able. See more at [How to clone a struct storing a boxed trait object?](https://stackoverflow.com/questions/30353462). Another choice is to change `Box<dyn Command>` to `Rc<RefCell<dyn Command>>`.

# Discussion
- We can extend `Command Pattern` to `Macro Command`, which has a set of commands in one command.

```rust
struct MacroCommand {
    commands: Vec<Rc<RefCell<dyn Command>>>>,
}
impl Command for MacroCommand {
    fn execute(&mut self) {
       //... 
    }
}
```
- (queuing requests) `Command` give us a way to package a piece of computation (a receiver and a set of actions), and the computation itself may be invoked long after submitting. We can take this scenario and apply it to many useful applications, such as schedulers, thread pools, and job queues, to name a few. 