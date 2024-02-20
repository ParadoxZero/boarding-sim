use std::collections::{HashMap, HashSet};

use crate::person::{Person, PersonGenerator, Seat};

#[derive(Clone, Copy, Debug, Default)]
pub struct PlaneSize {
    pub rows: u32,
    pub cols: u32,
    pub seats_per_cols: u32,
}

#[derive(Clone, Debug, Default)]
pub struct Plane {
    pub size: PlaneSize,
    seats_map: HashSet<Seat>,
    pathways: Vec<Vec<Option<Person>>>,
    person_generator: PersonGenerator,
}

impl PlaneSize {
    pub fn get_pathways(&self) -> u32 {
        let cols = self.cols;
        // 3 -> 2
        // 4 -> 3
        // 2 -> 1
        // Always 1 less than number of cols
        if cols > 1 {
            cols - 1
        } else {
            1
        }
    }
}

impl Plane {
    const TIME_TAKEN_BAGGAGE: u32 = 3;

    pub fn new(size: PlaneSize, person_generator: PersonGenerator) -> Plane {
        let pathways = vec![vec![None; size.rows as usize]; size.get_pathways() as usize];
        println!("Pathways: {:?}", pathways);
        Plane {
            size,
            seats_map: HashSet::new(),
            pathways,
            person_generator,
        }
    }

    // Model people boarding to airplane, each person moves ahead 1 step per tick,
    // and when they reach their target row, they will take a random amount of time
    // to sit, blocking rest of the line.
    // Currently no cost around which seat the person is going to sit in the column
    pub fn tick(&mut self) -> Result<(), ()> {
        for i in 0..self.size.get_pathways() {
            // Iterate over the pathways checking if People can go to seats
            for row in (0..self.pathways[i as usize].len()).rev() {
                let Some(mut person) = &self.pathways[i as usize][row] else {
                    continue;
                };
                assert!(Plane::is_valid_pathway(i, person.seat.col));
                if row as u32 == person.seat.row {
                    // If person reached their seat's row
                    if person.time_left_to_sit == 0 {
                        // Person is finished with preparing to sit
                        assert!(
                            !self.seats_map.contains(&person.seat),
                            "The seat is already occupied!"
                        );
                        self.seats_map.insert(person.seat);
                        self.pathways[i as usize][row] = None;
                    } else {
                        person.time_left_to_sit -= 1;
                        self.pathways[i as usize][row] = Some(person);
                    }
                } else {
                    // If person didn't reach their seat, move ahead
                    assert!(
                        row as u32 != self.size.rows,
                        "Last person couldn't get to their seat!"
                    );
                    if !self.pathways[i as usize][row as usize + 1].is_none() {
                        continue;
                    }
                    self.pathways[i as usize][row as usize + 1] = Some(person);
                    self.pathways[i as usize][row as usize] = None;
                }
            }
        }
        Ok(())
    }

    pub fn add_passenger_to_line(&mut self, person: Person) -> bool {
        for i in self.get_pathway(person.seat.col) {
            if self.pathways[i as usize][0].is_none() {
                self.pathways[i as usize][0] = Some(person);
                return true;
            }
        }

        false
    }

    fn is_valid_pathway(pathway: u32, col: u32) -> bool {
        col == pathway || col == pathway + 1
    }

    fn get_pathway(&self, col: u32) -> [u32; 2] {
        if col > 1 {
            [col, col - 1]
        } else {
            [0, 0]
        }
    }
}
