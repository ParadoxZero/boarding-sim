mod airplane;
mod person;

use crate::airplane::{Plane, PlaneSize};
use clap::Parser;

// #[derive(PartialEq, Eq, Clone, Copy)]
// enum Distribution {
//     Random,
//     OddEven,
//     Sequential,
// }

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Number of rows in the plane
    #[arg(short, long, default_value_t = 10)]
    rows: u32,

    /// Number of columns in the plane
    #[arg(short, long, default_value_t = 1)]
    cols: u32,

    /// Number of seats per column
    #[arg(short, long, default_value_t = 3)]
    seats: u32
}

fn main() {
    let args = Cli::parse();
    println!("Starting simulation...");

    let size = PlaneSize {cols:args.cols, rows:args.rows, seats_per_cols:args.seats};
    let mut plane = Plane::new(size);
    let mut person_generator = person::PersonGenerator::new(args.rows, args.cols, args.seats, 100);
    let mut total_ticks = 0;
    while let Some(person) =  person_generator.next() {
        println!("Seat #{},{},{} entered the line.", person.seat.row, person.seat.col, person.seat.seat_id);
        while !plane.add_passenger_to_line(person) {
            plane.tick().expect("Something went wrong!");
            total_ticks += 1;
        }

    }

    println!("Total ticks: {}", total_ticks);
    
}
