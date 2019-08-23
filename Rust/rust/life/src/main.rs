use ndarray::prelude::*;
use rand::*;
use std::fmt::*;

const DIMS: (usize, usize) = (10, 20);

struct World(Array2<bool>);

impl World {
    fn new() -> Self {
        World(Array2::default(DIMS))
    }
}

impl Display for World {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        //print out board state
        for r in 0..DIMS.0 {
            for c in 0..DIMS.1 {
                if world[[r, c]] {
                    write!(f, "●");
                } else {
                    write!(f, "○");
                }
            }
            println!();
        }
        Ok(())
    }
}

fn main() {
    let mut rng = thread_rng();
    let mut w: World::new();
    //build random board state
    for cell in w.0.iter_mut() {
        *cell = rng.gen();
    }
    print!("{}", w);
}
