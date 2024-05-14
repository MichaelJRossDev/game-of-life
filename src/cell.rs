pub mod cell {
    pub struct Cell {
        pub live: bool
    }

    impl Cell {
        pub fn new(live: bool) -> Cell {
            Cell { live }
        }

        pub fn survives(&self, live_neighbor_count: u16) -> bool {
            if self.live {
                match live_neighbor_count {
                    0..=1 => false,
                    2..=3 => true,
                    4.. => false
                }
            } else {
                match live_neighbor_count {
                    3 => true,
                    _ => false
                }
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::cell::Cell;

    #[test]
    fn first_rule() { // underpopulation
        let live_cell = Cell::new(true);
        assert_eq!(live_cell.survives(0), false);
        assert_eq!(live_cell.survives(1), false);
    }

    #[test]
    fn second_rule() { // survival
        let live_cell = Cell::new(true);
        assert_eq!(live_cell.survives(2), true);
        assert_eq!(live_cell.survives(3), true);
    }

    #[test]
    fn third_rule() { // overpopulation
        let live_cell = Cell::new(true);
        assert_eq!(live_cell.survives(4), false);
        assert_eq!(live_cell.survives(5), false);
        assert_eq!(live_cell.survives(6), false);
        assert_eq!(live_cell.survives(7), false);
        assert_eq!(live_cell.survives(8), false);
    }
}
