use std::collections::HashSet;

#[derive(Hash, Default, PartialEq, Eq)]
struct Coord {
    x: i32,
    y: i32,
    z: i32,
    w: i32
}

#[derive(Default)]
struct State {
    board: HashSet<Coord>,
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
            match c {
                '.' => {},
                '#' => {
                    set_active(&mut state, Coord {x, y, z: 0, w: 0});                    
                },
                _ => panic!("Unrecognised char '{}'", c)
            };

            x += 1;
        }

        y += 1;
    }

    state
}

fn set_active(state: &mut State, coord: Coord) {
    if coord.x < state.minx { state.minx = coord.x };
    if coord.x > state.maxx { state.maxx = coord.x };
    if coord.y < state.miny { state.miny = coord.y };
    if coord.y > state.maxy { state.maxy = coord.y };
    if coord.z < state.minz { state.minz = coord.z };
    if coord.z > state.maxz { state.maxz = coord.z };
    if coord.w < state.minw { state.minw = coord.w };
    if coord.w > state.maxw { state.maxw = coord.w };

    state.board.insert(coord);
}

fn dump_state(desc: &str, state: &State) {
    println!("{}:", desc);
    println!("");

    let mut coord: Coord = Default::default();

    for w in state.minw..=state.maxw {
        coord.w = w;

        for z in state.minz..=state.maxz {
            coord.z = z;

            println!("w={} z={} (@ x={}, y={})", w, z, state.minx, state.miny);

            for y in state.miny..=state.maxy {
                coord.y = y;

                for x in state.minx..=state.maxx {
                    coord.x = x;

                    print!("{}", match state.board.get(&coord).is_some() {
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
                    let coord = Coord { x, y, z, w };

                    let cur_active = state.board.get(&coord).is_some();

                    let neig = count_neighbours(&state, &coord);

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

                    if now_active {
                        set_active(&mut new_state, coord);
                    }
                }
            }
        }
    }

    new_state
}

fn count_neighbours(state: &State, coord: &Coord) -> u32 {
    let mut neig = 0;

    let mut coordp: Coord = Default::default();

    for xp in coord.x - 1..=coord.x + 1 {
        coordp.x = xp;

        for yp in coord.y - 1..=coord.y + 1 {
            coordp.y = yp;

            for zp in coord.z - 1..=coord.z + 1 {
                coordp.z = zp;

                for wp in coord.w - 1..=coord.w + 1 {
                    coordp.w = wp;

                    if coordp == *coord { continue };

                    if state.board.get(&coordp).is_some() {
                        neig += 1
                    };
                }
            }
        }
    }

    neig
}
