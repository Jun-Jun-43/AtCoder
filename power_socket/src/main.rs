use std::io;
use std::num::IntErrorKind::PosOverflow;
use std::str::FromStr;

use anyhow::Result;
use thiserror::Error;

#[derive(Error, Debug)]
enum InputError {
    #[error("Not proper input.")]
    NotProperInput,
}

fn calculation_of_required_power_sockets(
    number_of_plugs_in_power_strip: u8,
    number_of_required_plugs: u8,
) -> Result<u8> {
    let mut number_of_power_strips_required: u8 = 1;

    while number_of_power_strips_required <= number_of_required_plugs {
        number_of_power_strips_required += number_of_plugs_in_power_strip;
    }

    let result_number: u8 = number_of_power_strips_required / number_of_plugs_in_power_strip;

    Ok(result_number)
}

fn entry_field() -> Result<(u8, u8), InputError> {
    let mut number_of_plugs_in_power_strip = String::new();
    let mut number_of_required_plugs = String::new();

    println!("Enter the number of plugs on the power strip.");
    io::stdin()
        .read_line(&mut number_of_plugs_in_power_strip)
        .unwrap();

    println!("Please enter the number of inserts required.");
    io::stdin()
        .read_line(&mut number_of_required_plugs)
        .unwrap();

    let number_of_plugs_in_power_strip_u8: u8;
    let number_of_required_plugs_u8: u8;

    match u8::from_str(str::trim(&number_of_plugs_in_power_strip)) {
        Ok(u8) => {
            if !(&u8 > &255_u8 && &u8 < &1_u8) {
                number_of_plugs_in_power_strip_u8 = u8
            } else {
                return Err(InputError::NotProperInput);
            }
        }
        Err(err) => {
            if err.kind() == &PosOverflow {
                return Err(InputError::NotProperInput);
            } else {
                panic!("{:?}", err)
            }
        }
    }

    match u8::from_str(str::trim(&number_of_required_plugs)) {
        Ok(u8) => {
            if !(&u8 > &255_u8 && &u8 < &1_u8) {
                number_of_required_plugs_u8 = u8
            } else {
                return Err(InputError::NotProperInput);
            }
        }
        Err(err) => {
            if err.kind() == &PosOverflow {
                return Err(InputError::NotProperInput);
            } else {
                panic!("{:?}", err)
            }
        }
    }

    Ok((
        number_of_plugs_in_power_strip_u8,
        number_of_required_plugs_u8,
    ))
}

fn main() -> Result<()> {
    let (number_of_plugs_in_power_strip, number_of_required_plugs) = entry_field().unwrap();

    let required_power_sockets = calculation_of_required_power_sockets(
        number_of_plugs_in_power_strip,
        number_of_required_plugs,
    )
    .unwrap();

    println!("{} power sockets required.", required_power_sockets);

    Ok(())
}
