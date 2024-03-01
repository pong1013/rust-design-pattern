## Command 1
Implement simple remote with with only one bottun.
- Remote
    ```Rust
    pub struct SimpleRemoteControl {
        slot: Box<dyn Command>,
    }
    ```
- Device
    ```Rust
    pub struct Light;
    impl Light {
        pub fn on(&self) {
            println!("Light is on");
        }
    }
    ```
- Command
    ```Rust
    pub trait Command {
        fn execute(&self);
    }
    pub struct LightOnCommand {
        light: Light,
    }

    impl LightOnCommand {
        pub fn new(light: Light) -> Self {
            LightOnCommand { light }
        }
    }

    impl Command for LightOnCommand {
        fn execute(&self) {
            self.light.on();
        }
    }
    ```
- client (main)
    ```Rust
    fn main() {
        let mut remote = SimpleRemoteControl::new();
        let light_on = LightOnCommand::new(light);
        

        remote.set_command(Box::new(light_on));
        remote.button_pressed();
    }

    ```
## Command 2
Implement complex remoter with on&off button of each device. 
- Remote control has multiple slots
    ```Rust
    pub struct RemoteControl {
        on_commands: Vec<Box<dyn Command>>,
        off_commands: Vec<Box<dyn Command>>,
    }
    impl RemoteControl { 
            pub fn new() -> Self { ... }
            pub fn command_mut(
                &mut self, slot: usize, on: Box<dyn Command>, off: Box<dyn Command>
            ) { ... }
            pub fn press_on(&mut self, slot: usize) { ... }
            pub fn press_off(&mut self, slot: usize) { ... }
    }
    ```
    Show slots of remoter
    ```Rust
    impl fmt::Display for RemoteControl {...}
    ```
- device
    ```Rust
    pub struct Light {
        location: String,
    }

    impl Light {
        pub fn new(location: &str) -> Self {
            Light {
                location: String::from(location),
            }
        }
    ```
 -  Command `Box<T>.borrow()`
    ```Rust
    pub struct FanOnCommand {
        fan: Rc<RefCell<Fan>>,
    }
    impl FanOnCommand {
        pub fn new(fan: Rc<RefCell<Fan>>) -> Self {...}
    }
    impl Command for FanOnCommand {
        fn execute(&self) {...}
        fn name(&self) -> String {...}
    }
    ```
- client (main)
    ```Rust
    fn main() {
            let light = Rc::new(RefCell::new(Light::new("Living Room")));
            let light_on = LightOnCommand::new(Rc::clone(&light));
            let light_off = LightOffCommand::new(Rc::clone(&light));
            remote.command_mut(0, Box::new(light_on), Box::new(light_off));
    }
    ```
## Command 3
 
### Implement `undo()` button

1. Since `Box<dyn Command>` does not implement the `Clone` trait, we cannot move `on_commands[slot]` into the `undo_command` field.
2. If we change `undo_command` to store a reference, we need to add ***lifetime annotations***, otherwise the compiler cannot determine how long `undo_command` should live!
3. Considering the above, the simplest solution seems to be to convert `undo_command` into shareable data `Rc<T>`; additionally, since `execute()` and `undo()` have been modified to mutable methods, `undo_command` must have ***internal mutability*** as well!

- remote.rs
    ```Rust
    pub struct RemoteControl {
            on_commands: Vec<Rc<RefCell<dyn Command>>>,
        off_commands: Vec<Rc<RefCell<dyn Command>>>,
        undo_command: Rc<RefCell<dyn Command>>,
    }
    impl RemoteControl { 
            // ...

            pub fn press_on(&mut self, slot: usize) { 
                    self.on_commands[slot].as_mut().execute();
                    self.undo_command = Rc::clone(&self.on_commands[slot]);
            }
        pub fn press_off(&mut self, slot: usize) { ... }
            pub fn press_undo(&mut self) {
                    self.undo_command.borrow_mut().undo();
            }
    }
    ```

### Implement MacroCommand

- remote.rs
    ```Rust
    pub struct MacroCommand { 
        commands: Vec<Rc<RefCell<dyn Command>>> }
    impl MacroCommand {
        pub fn new(commands: Vec<Rc<RefCell<dyn Command>>>) -> Self { ... }
    }
    impl Command for MacroCommand {
        fn execute(&mut self) {
            for command in &self.commands {
                command.borrow_mut().execute();
            }
        }
        fn undo(&mut self) { // similar }
    }
    ...
    ```