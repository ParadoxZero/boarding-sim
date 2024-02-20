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
    pub fn generate_people(
        max_row: u32,
        max_col: u32,
        max_seat: u32,
        loading_factor: u32,
    ) -> Vec<Person> {
        assert!(loading_factor <= 100);
        let mut seats_map: HashSet<Seat> = HashSet::new();
        let total_people = max_col * max_row * max_seat * loading_factor / 100;
        let mut i = 0;
        while i <= total_people {
            let seat = Seat {
                row: random::<u32>() % max_row,
                col: random::<u32>() % max_col,
                seat_id: random::<u32>() % max_seat,
            };
            if !seats_map.contains(&seat) {
                seats_map.insert(seat);
                i += 1;
            }
        }
        seats_map
            .into_iter()
            .map(|seat| Person {
                seat,
                time_left_to_sit: random::<u32>() % Person::MAX_TIME_PER_PERSON,
            })
            .collect()
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct PersonGenerator {
    seat_map: HashSet<Seat>,
    max_row: u32,
    max_col: u32,
    max_seat: u32,
    loading_factor: u32,
}

impl PersonGenerator {
    pub fn new(max_row: u32, max_col: u32, max_seat: u32, loading_factor: u32) -> PersonGenerator {
        PersonGenerator {
            seat_map: HashSet::new(),
            max_row,
            max_col,
            max_seat,
            loading_factor,
        }
    }

    pub fn next(&mut self) -> Option<Person> {
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
