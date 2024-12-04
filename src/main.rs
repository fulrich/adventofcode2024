mod locations;
mod power_plant;

fn main() {
    println!("");
    println!("Day 1");
    locations::part_one();
    locations::part_two();

    power_plant::load_power();
}

