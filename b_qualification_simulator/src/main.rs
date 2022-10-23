use std::io;
use std::str::FromStr;

use anyhow::Result;
use rand::prelude::*;
use thiserror::Error;

#[derive(Error, Debug)]
enum QualificationSimulatorError {
    #[error("Total number of contest participants is less than 1.")]
    TheNumberOfContestParticipantsIsEnteredIncorrectly,
    #[error(
        "Input values for contest participants are out of range. contest participants <= 100000"
    )]
    InputValuesForContestParticipantsOutOfRange,
    #[error("Input values for qualifiers are out of range. Qualifiers <= 100000")]
    InputValuesForQualifiersOutOfRange,
    #[error("The number of qualifiers exceeds the number of contestants.")]
    IncorrectInputValuesForQualifiers,
}

fn input() -> Result<(usize, usize, usize), QualificationSimulatorError> {
    let mut input_total_contest_participants = String::new();
    let mut input_number_of_qualifiers_a = String::new();
    let mut input_number_of_qualifiers_b = String::new();

    println!(
        "{}",
        "Enter the number of participants in the contest (less than 100000)."
    );
    io::stdin()
        .read_line(&mut input_total_contest_participants)
        .unwrap();

    println!(
        "{}",
        "Enter the number of qualifiers A (less than or equal to the number of contestants)."
    );
    io::stdin()
        .read_line(&mut input_number_of_qualifiers_a)
        .unwrap();

    println!(
        "{}",
        "Enter the number of qualifiers B (less than or equal to the number of contestants)"
    );
    io::stdin()
        .read_line(&mut input_number_of_qualifiers_b)
        .unwrap();

    let total_contest_participants: usize =
        usize::from_str(&input_total_contest_participants.trim()).unwrap();
    let number_of_qualifiers_a: usize =
        usize::from_str(&input_number_of_qualifiers_a.trim()).unwrap();
    let number_of_qualifiers_b: usize =
        usize::from_str(&input_number_of_qualifiers_b.trim()).unwrap();

    if !(1 <= *&total_contest_participants) {
        return Err(
            QualificationSimulatorError::TheNumberOfContestParticipantsIsEnteredIncorrectly,
        );
    }
    if !(*&total_contest_participants <= 100000) {
        return Err(QualificationSimulatorError::InputValuesForContestParticipantsOutOfRange);
    }
    if !(*&number_of_qualifiers_a <= 100000 || *&number_of_qualifiers_b <= 100000) {
        return Err(QualificationSimulatorError::InputValuesForQualifiersOutOfRange);
    }
    if !((&number_of_qualifiers_a + &number_of_qualifiers_b) <= *&total_contest_participants) {
        return Err(QualificationSimulatorError::IncorrectInputValuesForQualifiers);
    }

    Ok((
        total_contest_participants,
        number_of_qualifiers_a,
        number_of_qualifiers_b,
    ))
}

fn creating_participant_information(total_contest_participants: usize) -> Result<Vec<String>> {
    let mut information_on_contest_participants: Vec<String> = Vec::new();

    for _i in 0..total_contest_participants {
        let n = thread_rng().gen_range(0..3);

        if n == 0_i32 {
            information_on_contest_participants.push(String::from("a"));
        } else if n == 1_i32 {
            information_on_contest_participants.push(String::from("b"));
        } else {
            information_on_contest_participants.push(String::from("c"));
        }
    }

    Ok(information_on_contest_participants)
}

fn qualification_simulator(
    number_of_qualifiers_a: usize,
    number_of_qualifiers_b: usize,
    information_on_contest_participants: Vec<String>,
) -> Result<()> {
    let total_number_of_qualifiers: usize =
        number_of_qualifiers_a.clone() + number_of_qualifiers_b.clone();
    let mut count_the_qualifiers: usize = 0;
    let mut count_number_of_qualifiers_b: usize = 0;

    for participant in information_on_contest_participants.iter() {
        if count_the_qualifiers == total_number_of_qualifiers {
            println!("{} {}", "No.", participant);
        } else {
            if *participant == String::from("a") {
                count_the_qualifiers += 1;
                println!("{} {}", "Yes.", participant);
            } else if *participant == String::from("b")
                && count_number_of_qualifiers_b < number_of_qualifiers_b
            {
                count_the_qualifiers += 1;
                count_number_of_qualifiers_b += 1;
                println!("{} {}", "Yes.", participant);
            } else {
                println!("{} {}", "No.", participant);
            }
        }
    }
    Ok(())
}

fn main() -> Result<()> {
    let (total_number_of_qualifiers, number_of_qualifiers_a, number_of_qualifiers_b) =
        input().unwrap();

    let information_on_contest_participants =
        creating_participant_information(total_number_of_qualifiers).unwrap();

    qualification_simulator(
        number_of_qualifiers_a,
        number_of_qualifiers_b,
        information_on_contest_participants,
    ).unwrap();

    Ok(())
}
