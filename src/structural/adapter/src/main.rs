use adapter::*;

fn main() {
    let duck = MallardDuck;

    let turkey = WildTurkey;
    let turkey_adapter = Turkeyadapter::new(Box::new(turkey));

    println!("The Duck says...");
    duck.quack();
    println!("\nThe Turkey says...");
    turkey_adapter.quack();
}
