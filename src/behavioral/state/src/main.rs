use state::*;

fn main() {
    let mut machine = GumballMachine::new(5); // 5 gumballs

    machine.insert_quarter();
    machine.turn_crank();
    println!("----------");

    machine.insert_quarter();
    machine.eject_quarter();
    machine.turn_crank();
    println!("----------");

    machine.insert_quarter();
    machine.turn_crank();
    machine.insert_quarter();
    machine.turn_crank();
    println!("----------");

    machine.insert_quarter();
    machine.insert_quarter();
    machine.turn_crank();
    machine.insert_quarter();
    machine.turn_crank();
    machine.insert_quarter();
    machine.turn_crank();
    println!("----------");
}
