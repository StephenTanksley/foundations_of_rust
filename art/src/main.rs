use stephen_art::mix;
use stephen_art::PrimaryColor;

fn main() {
    let red: PrimaryColor = PrimaryColor::Red;
    let yellow: PrimaryColor = PrimaryColor::Yellow;

    mix(red, yellow);
}
