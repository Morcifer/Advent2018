mod file_utilities;

#[cfg(test)]
mod day_1;

mod day_2;

use crate::file_utilities::get_file_path;
use crate::day_2::run;

fn main() {
    let day = 2;
    let is_test = false;
    let has_two_test_files = true;

    for part in [1, 2] {
        println!(
            "Day {day} Part {part}: {}",
            run(
                get_file_path(
                    is_test,
                    day,
                    if is_test && has_two_test_files { Some(part.to_string()) } else { None },
                ),
                part
            ),
        );
    }
}
