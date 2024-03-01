use chocolate_mutex::chocolate::{state, ChocolateBoiler, INIT};

fn main() {
    unsafe {
        // Set INIT with an initialization function
        INIT = Some(|| state {
            empty: false,
            boiled: false,
        });
    }
    // Create an instance of ChocolateBoiler
    let boiler_instance = ChocolateBoiler::instance();

    // Now you can use the boiler_instance as needed
    boiler_instance.boil();
    boiler_instance.fill();
    boiler_instance.drain();

    // Print the status of the boiler
    println!(
        "Boiler status: empty={}, boiled={}",
        boiler_instance.is_empty(),
        boiler_instance.is_Boiled()
    );
}
