use art::mix;
use art::PrimaryColor;

fn main() {
    let c1 = PrimaryColor::Red;
    let c2 = PrimaryColor::Yellow;
    let cmix = mix(c1, c2);
    println!("{:?}", cmix);
}
