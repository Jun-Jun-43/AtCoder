use std::convert::TryFrom;
use std::io;
use std::str::FromStr;

use anyhow::Result;
use thiserror::Error;

#[derive(Error, Debug)]
enum InputError {
    #[error("The number of residents does not match the number of coordinates.")]
    MismatchBetweenNumberOfResidentsAndLengthOfCoordinates,
}

fn the_minimum_amount_of_physical_energy_a_resident_must_expend_to_attend_a_meeting(
    number_of_inhabitants: usize,
    dwelling_coordinates: Vec<i32>,
) -> Result<i32, InputError> {
    if &number_of_inhabitants != &dwelling_coordinates.len() {
        return Err(InputError::MismatchBetweenNumberOfResidentsAndLengthOfCoordinates);
    }

    let mut sum_of_physical_energy_consumed: i32 = 0;

    let mut coordinate: u8 = 0;

    while coordinate != 100 {
        let mut sample_of_physical_energy_consumed: i32 = 0;

        for resident in 0..number_of_inhabitants {
            sample_of_physical_energy_consumed +=
                (dwelling_coordinates[resident] - i32::try_from(coordinate.clone()).unwrap()).pow(2)
        }

        if sum_of_physical_energy_consumed == 0 {
            sum_of_physical_energy_consumed = sample_of_physical_energy_consumed;
        } else {
            if sum_of_physical_energy_consumed > sample_of_physical_energy_consumed {
                sum_of_physical_energy_consumed = sample_of_physical_energy_consumed;
            }
        }

        coordinate += 1;
    }

    Ok(sum_of_physical_energy_consumed)
}

fn main() -> Result<()> {
    let mut input_number_of_inhabitants = String::new();
    let mut input_dwelling_coordinates = String::new();

    println!("Enter the number of residents");
    io::stdin()
        .read_line(&mut input_number_of_inhabitants)
        .unwrap();

    println!("Enter the coordinates where each resident lives, separated by commas");
    io::stdin()
        .read_line(&mut input_dwelling_coordinates)
        .unwrap();


    let number_of_inhabitants: usize =
        usize::from_str(&input_number_of_inhabitants.trim()).unwrap();

    let dwelling_coordinates: Vec<i32> = input_dwelling_coordinates
        .trim()
        .rsplit(",")
        .map(|v| i32::from_str(v).unwrap())
        .collect();

    let minimum_physical_energy_to_be_consumed = the_minimum_amount_of_physical_energy_a_resident_must_expend_to_attend_a_meeting(number_of_inhabitants, dwelling_coordinates).unwrap();

    println!(
        "The minimum total sum of energy to be consumed is {:?}", minimum_physical_energy_to_be_consumed);

    Ok(())
}
