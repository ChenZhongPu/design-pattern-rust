use facade::{Amplifier, Tuner, HomeTheaterFacade};

fn main() {
    let amp = Amplifier::new("Amplifier");
    let tuner = Tuner::new("AM/FM Tuner");

    let mut home_theater = HomeTheaterFacade::new(amp, tuner);
    home_theater.listen_radio(64.0);
    home_theater.stop_radio();
}