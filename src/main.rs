mod gol;
use gol::*;

fn main() {
    //let world1: World = World::new_empty_world();
    //println!("{}", world1);

    let mut world2: World = World::new_world(
        [
            [0, 0, 0, 0, 0, 0],
            [0, 0, 1, 0, 0, 0],
            [0, 1, 1, 0, 0, 0],
            [0, 0, 0, 1, 0, 0],
            [0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0]
        ]
    );

    println!("{}", world2);
    for i in 1..5 {
        world2.step_forward();
        println!("{}", world2);
    }
}
