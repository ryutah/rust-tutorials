extern crate closure;

use closure::generate_workout;

fn main() {
    let simulated_user_specified = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified, simulated_random_number);
}
