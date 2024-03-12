use facade::{Amplifier, DVDplayer, HomeTheaterFacade, TV};
fn main() {
    let amp = Amplifier::new("Amplifier");
    let dvdplayer = DVDplayer::new("Fast&Furious20");
    let tv = TV::new("85 inches TV");

    let mut theater = HomeTheaterFacade::new(amp, dvdplayer, tv);
    println!("---------------");
    theater.open_theater();
    println!("----------------");
    theater.stop_theater();
}
