use std::collections::HashSet;

#[derive(Default)]
struct State {
    board: HashSet<String>,
    minx: i32,
    maxx: i32,
    miny: i32,
    maxy: i32,
    minz: i32,
    maxz: i32,
    minw: i32,
    maxw: i32
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut state = setup_state("\
        ..#....#\n\
        ##.#..##\n\
        .###....\n\
        #....#.#\n\
        #.######\n\
        ##.#....\n\
        #.......\n\
        .#......\n\
    ");

    dump_state("Initial", &state);

    for iter in 0..6 {
        state = mutate_state(state);

        dump_state(&format!("After cycle {}", iter + 1), &state);
    }

    println!("{} cubes active", state.board.iter().count());

    Ok(())
}

fn setup_state(board: &str) -> State {
    let mut state = Default::default();

    let mut y = 0;

    for line in board.lines() {
        let mut x = 0;

        for c in line.chars() {
            let active = match c {
                '.' => false,
                '#' => true,
                _ => panic!("Unrecognised char '{}'", c)
            };

            set_state(&mut state, x, y, 0, 0, active);

            x += 1;
        }

        y += 1;
    }

    state
}

fn set_state(state: &mut State, x: i32, y: i32, z: i32, w: i32, active: bool) {
    if active {
        state.board.insert(coord_to_key(x, y, z, w));

        if x < state.minx { state.minx = x };
        if x > state.maxx { state.maxx = x };
        if y < state.miny { state.miny = y };
        if y > state.maxy { state.maxy = y };
        if z < state.minz { state.minz = z };
        if z > state.maxz { state.maxz = z };
        if w < state.minw { state.minw = w };
        if w > state.maxw { state.maxw = w };
    }
}

fn coord_to_key(x: i32, y: i32, z: i32, w: i32) -> String {
    format!("{},{},{},{}", x, y, z, w)
}

fn dump_state(desc: &str, state: &State) {
    println!("{}:", desc);
    println!("");

    for w in state.minw..=state.maxw {
        for z in state.minz..=state.maxz {
            println!("w={} z={} (@ x={}, y={})", w, z, state.minx, state.miny);

            for y in state.miny..=state.maxy {
                for x in state.minx..=state.maxx {
                    print!("{}", match state.board.get(&coord_to_key(x, y, z, w)).is_some() {
                        true => '#',
                        false => '.'
                    });
                }

                println!("");
            }

            println!("");
        }
    }
}

fn mutate_state(state: State) -> State {
    let mut new_state = Default::default();

    for w in state.minw - 1..=state.maxw + 1 {
        for z in state.minz - 1..=state.maxz + 1 {
            for y in state.miny - 1..=state.maxy + 1 {
                for x in state.minx - 1..=state.maxx + 1 {
                    let cur_active = state.board.get(&coord_to_key(x, y, z, w)).is_some();

                    let neig = count_neighbours(&state, x, y, z, w);

                    let now_active = if cur_active {
                        match neig {
                            2 | 3 => true,
                            _ => false
                        }
                    } else{
                        match neig {
                            3 => true,
                            _ => false
                        }
                    };

                    set_state(&mut new_state, x, y, z, w, now_active);
                }
            }
        }
    }

    new_state
}

fn count_neighbours(state: &State, x: i32, y: i32, z: i32, w: i32) -> u32 {
    let mut neig = 0;

    for xp in x - 1..=x + 1 {
        for yp in y - 1..=y + 1 {
            for zp in z - 1..=z + 1 {
                for wp in w - 1..=w + 1 {
                    if xp == x && yp == y && zp == z && wp == w { continue };

                    if state.board.get(&coord_to_key(xp, yp, zp, wp)).is_some() {
                        neig += 1
                    };
                }
            }
        }
    }

    neig
}
