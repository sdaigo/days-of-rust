fn main() {
    const DISTANCE: i32 = 384_400;
    let velocity_car = 80;
    let velocity_train = 300;

    println!(
        "{} days to the moon with a car",
        DISTANCE / velocity_car / 24
    );
    println!(
        "{} days to the moon with a train",
        DISTANCE / velocity_train / 24
    );
}
