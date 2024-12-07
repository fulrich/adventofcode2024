mod locations;
mod power_plant;
mod toboggan_rental;

fn main() {
    locations::parse_locations();
    power_plant::load_power();
    toboggan_rental::corrupted_data();
}

