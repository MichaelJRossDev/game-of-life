use crate::cell::cell::Cell;

mod cell;

fn main() {
    let my_cell = Cell::new(true);
    println!("My cell is live: {}", my_cell.live);
}

