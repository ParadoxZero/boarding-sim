use std::collections::HashSet;

use rand::random;

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub struct Seat {
    pub row: u32,
    pub col: u32,
    pub seat_id: u32,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Person {
    pub seat: Seat,
    pub time_left_to_sit: u32,
}

impl Person {
    const MAX_TIME_PER_PERSON: u32 = 5;
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct RandomPersonGenerator {
    seat_map: HashSet<Seat>,
    max_row: u32,
    max_col: u32,
    max_seat: u32,
    loading_factor: u32,
}

pub trait PersonGenerator {
    fn next(&mut self) -> Option<Person>;
}

impl RandomPersonGenerator {
    pub fn new(
        max_row: u32,
        max_col: u32,
        max_seat: u32,
        loading_factor: u32,
    ) -> RandomPersonGenerator {
        RandomPersonGenerator {
            seat_map: HashSet::new(),
            max_row,
            max_col,
            max_seat,
            loading_factor,
        }
    }
    fn random_seat(&mut self) -> Seat {
        Seat {
            row: random::<u32>() % self.max_row,
            col: random::<u32>() % self.max_col,
            seat_id: random::<u32>() % self.max_seat,
        }
    }

    fn total_allowed_people(&self) -> u32 {
        self.max_row * self.max_col * self.max_seat * self.loading_factor / 100
    }
}
impl PersonGenerator for RandomPersonGenerator {
    fn next(&mut self) -> Option<Person> {
        if self.seat_map.len() >= self.total_allowed_people().try_into().unwrap() {
            return None;
        }

        let mut seat = self.random_seat();
        while self.seat_map.contains(&seat) {
            seat = self.random_seat();
        }

        self.seat_map.insert(seat);
        Some(Person {
            seat,
            time_left_to_sit: random::<u32>() % Person::MAX_TIME_PER_PERSON,
        })
    }
}
