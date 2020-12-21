use std::fs;
use std::io::{self, BufRead};
use std::str;
use std::cmp;
use std::collections::HashMap;

const TILE_DIM: usize = 10;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Edge {
    North = 0,
    East = 1,
    South = 2,
    West = 3
}

type TileNum = u16;

#[derive(Default)]
struct Tile {
    tile_num: TileNum,          // Tile number
    tile_data: [u16; TILE_DIM], // Raw tile data
    edge_norm: [u16; 4],        // Normalised edge (lowest of forward and reverse bits)
    edge: [u16; 4],             // Actual edge value
    east: TileNum,              // The tile to the east
    south: TileNum              // The tile to the south
}

type TileMap = HashMap<TileNum, Tile>;

type EdgeMap = HashMap<u16, Vec<TileNum>>;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut tiles = load_tiles()?;

    let edges = build_edge_lookup(&tiles);

    let corner = arrange_tiles(&mut tiles, &edges);

    let mut map = build_map(&tiles, corner);

    let found = find_monsters(&mut map);

    println!("Found {} monsters", found);

    print_map(&map);

    let roughness = count_hash(&map);

    println!("Roughness is {}", roughness);

    Ok(())
}

fn load_tiles() -> Result<TileMap, Box<dyn std::error::Error>> {
    // Open the file read only
    let input = fs::File::open("../input20.txt")?;

    // Create a buffered reader on the file
    let inputbuf = io::BufReader::new(input);

    let mut tiles = HashMap::new();

    enum LoadStage {
        TileNum,
        TileData,
        Blank
    }
    
    let mut load_stage = LoadStage::TileNum;

    let mut tile: Tile = Default::default();
    let mut data_line: usize = 0;

    for line_result in inputbuf.lines() {
        let line = line_result?;

        match load_stage {
            LoadStage::TileNum => {
                tile.tile_num = line.split_whitespace().nth(1).unwrap()
                    .split(":").next().unwrap()
                    .parse::<u16>().unwrap();

                load_stage = LoadStage::TileData;
                data_line = 0;
            },
            LoadStage::TileData => {
                tile.tile_data[data_line] = string_to_int(&line)?;

                data_line += 1;

                if data_line == TILE_DIM {
                    calc_edges(&mut tile);

                    tiles.insert(tile.tile_num, tile);

                    tile = Default::default();

                    load_stage = LoadStage::Blank;
                }
            }
            LoadStage::Blank => {
                if !line.is_empty() {
                    Err("Expecting blank line")?
                }

                load_stage = LoadStage::TileNum
            }
        }
    }

    Ok(tiles)
}

fn string_to_int(string: &str) -> Result<u16, Box<dyn std::error::Error>> {
    let mut result: u16 = 0;

    let mut bit = 1 << (TILE_DIM - 1);

    for c in string.chars() {
        match c {
            '.' => {},
            '#' => {
                result += bit;
            },
            _ => {
                Err(format!("Invalid character {}", c))?
            }
        }

        bit >>= 1;
    }

    Ok(result)
}

fn calc_edges(tile: &mut Tile) {
    tile.edge[Edge::North as usize] = tile.tile_data[0];
    tile.edge[Edge::South as usize] = tile.tile_data[TILE_DIM - 1];

    tile.edge_norm[Edge::North as usize] = calc_edge(tile.tile_data[0]);
    tile.edge_norm[Edge::South as usize] = calc_edge(tile.tile_data[TILE_DIM - 1]);

    let mut e: u16 = 0;
    let mut w: u16 = 0;

    let emask = 0x01;
    let wmask = 1 << (TILE_DIM - 1);

    let mut bit = 1 << (TILE_DIM - 1);
    
    for i in 0..TILE_DIM {
        if tile.tile_data[i] & emask != 0 {
            e += bit;
        }

        if tile.tile_data[i] & wmask != 0 {
            w += bit;
        }

        bit >>= 1;
    }

    tile.edge[Edge::East as usize] = e;
    tile.edge[Edge::West as usize] = w;

    tile.edge_norm[Edge::East as usize] = calc_edge(e);
    tile.edge_norm[Edge::West as usize] = calc_edge(w);
}

#[test]
fn test_calc_edges() {
    let mut tile: Tile = Default::default();

    tile.tile_data[0] = 0b1000110000;
    tile.tile_data[1] = 0b0001111000;
    tile.tile_data[2] = 0b0011111100;
    tile.tile_data[3] = 0b0110110110;
    tile.tile_data[4] = 0b1100110011;
    tile.tile_data[5] = 0b0000110000;
    tile.tile_data[6] = 0b0000110000;
    tile.tile_data[7] = 0b0001111000;
    tile.tile_data[8] = 0b0011001100;
    tile.tile_data[9] = 0b0110000111;

    calc_edges(&mut tile);

    assert!(tile.edge[Edge::North as usize] == 0b1000110000, "North edge incorrect");
    assert!(tile.edge_norm[Edge::North as usize] == 0b0000110001, "Norm north edge incorrect");
    assert!(tile.edge[Edge::East as usize] == 0b0000100001, "East edge incorrect");
    assert!(tile.edge_norm[Edge::East as usize] == 0b100001, "Norm east edge incorrect");
    assert!(tile.edge[Edge::South as usize] == 0b0110000111, "South edge incorrect");
    assert!(tile.edge_norm[Edge::South as usize] == 0b0110000111, "Norm south edge incorrect");
    assert!(tile.edge[Edge::West as usize] == 0b1000100000, "West edge incorrect");
    assert!(tile.edge_norm[Edge::West as usize] == 0b0000010001, "Norm west edge incorrect");
}

fn calc_edge(edge: u16) -> u16 {
    cmp::min(edge, reverse_bits(edge))
}

fn reverse_bits(int: u16) -> u16 {
    let mut bit_in: u16 = 1 << (TILE_DIM - 1);
    let mut bit_out: u16 = 1;
    let mut result: u16 = 0;

    for _ in 0..TILE_DIM {
        if int & bit_in != 0 {
            result += bit_out
        }

        bit_in >>= 1;
        bit_out <<= 1;
    }

    result
}

fn build_edge_lookup(tiles: &TileMap) -> EdgeMap {
    let mut map = HashMap::new();

    let mut add = |edge_val: u16, tile: &Tile| {
        match map.get_mut(&edge_val) {
            None => {
                let mut coll: Vec<u16> = Vec::new();
                coll.push(tile.tile_num);
                map.insert(edge_val, coll);
            }
            Some(coll) => {
                coll.push(tile.tile_num);
            }
        };
    };

    for (_tile_num, tile) in tiles {
        add(tile.edge_norm[Edge::North as usize], tile);
        add(tile.edge_norm[Edge::East as usize], tile);
        add(tile.edge_norm[Edge::South as usize], tile);
        add(tile.edge_norm[Edge::West as usize], tile);
    }

    map
}

fn get_outside_edges(tile: &Tile, edges: &EdgeMap) -> Vec<Edge> {
    let mut outside: Vec<Edge> = Vec::new();

    let mut calc = |edge: Edge| {
        let edge_val = tile.edge_norm[edge as usize];

        if let Some(vec) = edges.get(&edge_val) {
            if vec.len() == 1 {
                outside.push(edge);
                return true;
            }
        }

        false
    };

    if !calc(Edge::North) {
        calc(Edge::South);
    }
    if !calc(Edge::East) {
        calc(Edge::West);
    }

    outside
}

fn arrange_tiles(tiles: &mut TileMap, edges: &EdgeMap) -> TileNum {
    // Find a corner
    let (corner_tile_num, outside_edges) = find_corner(tiles, edges).expect("Unable to find corner tile");

    let corner_tile = tiles.get_mut(&corner_tile_num).unwrap();

    // Orient so edges are North and West
    if outside_edges[0] == Edge::South {
        flip_vert(corner_tile);
    }

    if outside_edges[1] == Edge::East {
        flip_horiz(corner_tile);
    }

    let test_orient = get_outside_edges(corner_tile, &edges);
    assert!(test_orient == vec![Edge::North, Edge::West], format!("Orientation failed - {:?}", test_orient));

    // Set current left tile number
    let mut left_tile_num = corner_tile_num;

    loop {
        // Work eastwards
        walk_east(left_tile_num, tiles, edges);

        // Step south            
        match step_south(left_tile_num, tiles, edges) {
            Some(next) => left_tile_num = next,
            None => break
        }
    }

    corner_tile_num
}

fn find_corner(tiles: &TileMap, edges: &EdgeMap) -> Option<(u16, Vec<Edge>)> {
    let mut corners = Vec::new();

    // Find a corner
    for (&tile_num, tile) in tiles {
        let outside = get_outside_edges(&tile, &edges);

        if outside.len() == 2 {
            corners.push(tile_num);
        }
    }

    if corners.len() == 0 {
        None
    } else{
        corners.sort();
        let first = corners[0];
        let tile = tiles.get(&first).unwrap();
        let outside = get_outside_edges(&tile, &edges);
        Some((first, outside))
    }
}

fn find_tile(from: &Tile, edge: Edge, edges: &EdgeMap) -> Option<TileNum> {
    let edge_val = from.edge_norm[edge as usize];

    if let Some(vec) = edges.get(&edge_val) {
        let filtvec: Vec<&TileNum> = vec.iter().filter(|&&tile_num| tile_num != from.tile_num).collect();

        match filtvec.len() {
            0 => None,
            1 => Some(*filtvec[0]),
            _ => panic!("Multiple tiles found")
        }
    } else {
        None
    }
}

fn walk_east(tile_num: TileNum, tiles: &mut TileMap, edges: &EdgeMap) {
    let mut cur_tile_num = tile_num;
    let mut cur_east_tile_num: TileNum;
    let mut cur_east: u16;
    let mut cur_east_norm: u16;

    print!("East from {}", tile_num);

    loop {
        // Find out which tile is to the east
        match tiles.get(&cur_tile_num) {
            Some(cur_tile) => {
                match find_tile(&cur_tile, Edge::East, &edges) {
                    Some(t) => {
                        cur_east_tile_num = t;
                        cur_east = cur_tile.edge[Edge::East as usize];
                        cur_east_norm = cur_tile.edge_norm[Edge::East as usize];
                    },
                    None => break
                }
            },
            None => panic!("Unable to find tile {}", cur_tile_num)
        };

        // Update current tile
        if let Some(mut cur_tile) = tiles.get_mut(&cur_tile_num) {
            cur_tile.east = cur_east_tile_num;
        } else {
            panic!("Unable to get current tile");
        }

        // Update tile to the east
        if let Some(mut east_tile) = tiles.get_mut(&cur_east_tile_num) {
            // Move matching side to the east side of this tile
            match find_side_norm(&east_tile, cur_east_norm) {
                Edge::North => rotate_ccw(&mut east_tile),
                Edge::East => flip_horiz(&mut east_tile),
                Edge::South => rotate_cw(&mut east_tile),
                Edge::West => {}
            }

            // Flip vertically if necessary
            if cur_east != east_tile.edge[Edge::West as usize] {
                flip_vert(&mut east_tile);
            }
            assert!(cur_east == east_tile.edge[Edge::West as usize], "East sides don't match");
        } else {
            panic!("Unable to get east tile")
        }

        // Move to the east tile
        cur_tile_num = cur_east_tile_num;

        print!(" -> {}", cur_tile_num);
    }

    println!("");
}

fn step_south(tile_num: TileNum, tiles: &mut TileMap, edges: &EdgeMap) -> Option<TileNum> {
    let south_tile_num: TileNum;
    let south: u16;
    let south_norm: u16;

    // Find out which tile is to the south
    match tiles.get(&tile_num) {
        Some(cur_tile) => {
            match find_tile(&cur_tile, Edge::South, &edges) {
                Some(t) => {
                    south_tile_num = t;
                    south = cur_tile.edge[Edge::South as usize];
                    south_norm = cur_tile.edge_norm[Edge::South as usize];
                },
                None => return None
            }
        },
        None => panic!("Unable to find tile {}", tile_num)
    };

    // Update current tile
    if let Some(mut cur_tile) = tiles.get_mut(&tile_num) {
        cur_tile.south = south_tile_num;
    };

    // Update tile to the south
    if let Some(mut south_tile) = tiles.get_mut(&south_tile_num) {
        // Move matching side to the south side of this tile
        match find_side_norm(&south_tile, south_norm) {
            Edge::North => {},
            Edge::East => rotate_ccw(&mut south_tile),
            Edge::South => flip_vert(&mut south_tile),
            Edge::West => rotate_cw(&mut south_tile)
        }

        // Flip vertically if necessary
        if south != south_tile.edge[Edge::North as usize] {
            flip_horiz(&mut south_tile);
        }
        assert!(south == south_tile.edge[Edge::North as usize], "South sides don't match");
    };

    Some(south_tile_num)
}

fn find_side_norm(tile: &Tile, side: u16) -> Edge {
    if tile.edge_norm[Edge::North as usize] == side {
        return Edge::North
    } else if tile.edge_norm[Edge::East as usize] == side {
        return Edge::East
    } else if tile.edge_norm[Edge::South as usize] == side {
        return Edge::South
    } else if tile.edge_norm[Edge::West as usize] == side {
        return Edge::West
    } else {
        panic!("Edge not found")
    }
}

fn flip_horiz(tile: &mut Tile) {
    for i in 0..TILE_DIM {
        tile.tile_data[i] = reverse_bits(tile.tile_data[i]);
    }

    let tmp1 = tile.edge_norm[Edge::West as usize];
    tile.edge_norm[Edge::West as usize] = tile.edge_norm[Edge::East as usize];
    tile.edge_norm[Edge::East as usize] = tmp1;

    let tmp2 = tile.edge[Edge::West as usize];
    tile.edge[Edge::West as usize] = tile.edge[Edge::East as usize];
    tile.edge[Edge::East as usize] = tmp2;

    tile.edge[Edge::North as usize] = reverse_bits(tile.edge[Edge::North as usize]);
    tile.edge[Edge::South as usize] = reverse_bits(tile.edge[Edge::South as usize]);
}

fn flip_vert(tile: &mut Tile) {
    let mut save_data = [0; TILE_DIM];

    for i in 0..TILE_DIM {
        save_data[i] = tile.tile_data[i];
    }

    for i in 0..TILE_DIM {
        tile.tile_data[i] = save_data[TILE_DIM - 1 - i];
    }

    let tmp1 = tile.edge_norm[Edge::North as usize];
    tile.edge_norm[Edge::North as usize] = tile.edge_norm[Edge::South as usize];
    tile.edge_norm[Edge::South as usize] = tmp1;

    let tmp2 = tile.edge[Edge::North as usize];
    tile.edge[Edge::North as usize] = tile.edge[Edge::South as usize];
    tile.edge[Edge::South as usize] = tmp2;

    tile.edge[Edge::East as usize] = reverse_bits(tile.edge[Edge::East as usize]);
    tile.edge[Edge::West as usize] = reverse_bits(tile.edge[Edge::West as usize]);
}

#[test]
fn test_flip_vert() {
    let mut tile: Tile = Default::default();

    tile.tile_data[0] = 0b1000110000;
    tile.tile_data[1] = 0b0001111000;
    tile.tile_data[2] = 0b0011111100;
    tile.tile_data[3] = 0b0110110110;
    tile.tile_data[4] = 0b1100110011;
    tile.tile_data[5] = 0b0000110000;
    tile.tile_data[6] = 0b0000110000;
    tile.tile_data[7] = 0b0001111000;
    tile.tile_data[8] = 0b0011001100;
    tile.tile_data[9] = 0b0110000111;

    flip_vert(&mut tile);

    assert!(tile.tile_data[0] == 0b0110000111, "Elem 0 incorrect");
    assert!(tile.tile_data[1] == 0b0011001100, "Elem 1 incorrect");
    assert!(tile.tile_data[2] == 0b0001111000, "Elem 2 incorrect");
    assert!(tile.tile_data[3] == 0b0000110000, "Elem 3 incorrect");
    assert!(tile.tile_data[4] == 0b0000110000, "Elem 4 incorrect");
    assert!(tile.tile_data[5] == 0b1100110011, "Elem 5 incorrect");
    assert!(tile.tile_data[6] == 0b0110110110, "Elem 6 incorrect");
    assert!(tile.tile_data[7] == 0b0011111100, "Elem 7 incorrect");
    assert!(tile.tile_data[8] == 0b0001111000, "Elem 8 incorrect");
    assert!(tile.tile_data[9] == 0b1000110000, "Elem 9 incorrect");
}

fn rotate_cw(tile: &mut Tile) {
    let mut save_data = [0; TILE_DIM];

    for i in 0..TILE_DIM {
        save_data[i] = tile.tile_data[i];
    }

    // 2 1 0    8 5 2
    // 5 4 3 -> 7 4 1
    // 8 7 6    6 3 0
    
    let mut src_bit = 1 << (TILE_DIM - 1);

    for i in 0..TILE_DIM {
        let mut dst_bit = 1 << (TILE_DIM - 1);
        tile.tile_data[i] = 0;

        for j in 0..TILE_DIM {
            if save_data[TILE_DIM - 1 - j] & src_bit != 0 {
                tile.tile_data[i] += dst_bit;
            }

            dst_bit >>= 1;
        }

        src_bit >>= 1;
    }

    calc_edges(tile);
}

#[test]
fn test_rotate_cw() {
    let mut tile: Tile = Default::default();

    tile.tile_data[0] = 0b1000110000;
    tile.tile_data[1] = 0b0001111000;
    tile.tile_data[2] = 0b0011111100;
    tile.tile_data[3] = 0b0110110110;
    tile.tile_data[4] = 0b1100110011;
    tile.tile_data[5] = 0b0000110000;
    tile.tile_data[6] = 0b0000110000;
    tile.tile_data[7] = 0b0001111000;
    tile.tile_data[8] = 0b0011001100;
    tile.tile_data[9] = 0b0110000111;

    rotate_cw(&mut tile);

    assert!(tile.tile_data[0] == 0b0000010001, "Elem 0 incorrect");
    assert!(tile.tile_data[1] == 0b1000011000, "Elem 1 incorrect");
    assert!(tile.tile_data[2] == 0b1100001100, "Elem 2 incorrect");
    assert!(tile.tile_data[3] == 0b0110000110, "Elem 3 incorrect");
    assert!(tile.tile_data[4] == 0b0011111111, "Elem 4 incorrect");
    assert!(tile.tile_data[5] == 0b0011111111, "Elem 5 incorrect");
    assert!(tile.tile_data[6] == 0b0110000110, "Elem 6 incorrect");
    assert!(tile.tile_data[7] == 0b1100001100, "Elem 7 incorrect");
    assert!(tile.tile_data[8] == 0b1000011000, "Elem 8 incorrect");
    assert!(tile.tile_data[9] == 0b1000010000, "Elem 9 incorrect");
}

fn rotate_ccw(tile: &mut Tile) {
    let mut save_data = [0; TILE_DIM];

    for i in 0..TILE_DIM {
        save_data[i] = tile.tile_data[i];
    }

    // 2 1 0    0 3 6
    // 5 4 3 -> 1 4 7
    // 8 7 6    2 5 8
    
    let mut src_bit = 1;

    for i in 0..TILE_DIM {
        let mut dst_bit = 1 << (TILE_DIM - 1);
        tile.tile_data[i] = 0;

        for j in 0..TILE_DIM {
            if save_data[j] & src_bit != 0 {
                tile.tile_data[i] += dst_bit;
            }

            dst_bit >>= 1;
        }

        src_bit <<= 1;
    }

    calc_edges(tile);
}

#[test]
fn test_rotate_ccw() {
    let mut tile: Tile = Default::default();

    tile.tile_data[0] = 0b1000110000;
    tile.tile_data[1] = 0b0001111000;
    tile.tile_data[2] = 0b0011111100;
    tile.tile_data[3] = 0b0110110110;
    tile.tile_data[4] = 0b1100110011;
    tile.tile_data[5] = 0b0000110000;
    tile.tile_data[6] = 0b0000110000;
    tile.tile_data[7] = 0b0001111000;
    tile.tile_data[8] = 0b0011001100;
    tile.tile_data[9] = 0b0110000111;

    rotate_ccw(&mut tile);

    assert!(tile.tile_data[0] == 0b0000100001, "Elem 0 incorrect");
    assert!(tile.tile_data[1] == 0b0001100001, "Elem 1 incorrect");
    assert!(tile.tile_data[2] == 0b0011000011, "Elem 2 incorrect");
    assert!(tile.tile_data[3] == 0b0110000110, "Elem 3 incorrect");
    assert!(tile.tile_data[4] == 0b1111111100, "Elem 4 incorrect");
    assert!(tile.tile_data[5] == 0b1111111100, "Elem 5 incorrect");
    assert!(tile.tile_data[6] == 0b0110000110, "Elem 6 incorrect");
    assert!(tile.tile_data[7] == 0b0011000011, "Elem 7 incorrect");
    assert!(tile.tile_data[8] == 0b0001100001, "Elem 8 incorrect");
    assert!(tile.tile_data[9] == 0b1000100000, "Elem 9 incorrect");
}

fn build_map(tiles: &TileMap, corner: TileNum) -> Vec<Vec<char>> {
    let mut result: Vec<Vec<char>> = Vec::new();

    let mut cur_left_num = corner;
    let mut y = 0;

    while cur_left_num != 0 {
        let mut cur_tile_num = cur_left_num;

        for _ in y..y + (TILE_DIM - 2) {
            result.push(Vec::new());
        }

        while cur_tile_num != 0 {
            let tile = tiles.get(&cur_tile_num).unwrap();

            for i in 0..TILE_DIM - 2 {
                let val = tile.tile_data[i + 1];

                let mut bit = 1 << TILE_DIM - 2;

                for _ in 0..TILE_DIM - 2 {
                    result[y + i].push(if val & bit != 0 {
                        '#'
                    } else {
                        '.'
                    });

                    bit >>= 1;
                }
            }

            cur_tile_num = tile.east;
        }

        cur_left_num = tiles.get(&cur_left_num).unwrap().south;
        y += TILE_DIM - 2;
    }

    result
}

fn print_map(map: &Vec<Vec<char>>) {
    for line in map {
        println!("{}", line.iter().collect::<String>());
    }
}

// 01234567890123456789
//                   # 
// #    ##    ##    ###
//  #  #  #  #  #  #   

const MONSTER_OFFSETS: [[isize; 2]; 15]= [
    [0, 0], [1, 1],
    [3, 0], [1, -1], [1, 0], [1, 1],
    [3, 0], [1, -1], [1, 0], [1, 1],
    [3, 0], [1, -1], [1, 0], [0, -1], [1, 1]
];

fn find_monsters(map: &mut Vec<Vec<char>>) -> usize {
    let mut found = 0;

    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] == '#' {
                if find_monster(map, x as isize, y as isize) {
                    found += 1;
                }
            }
        }
    }

    found
}

fn find_monster(map: &mut Vec<Vec<char>>, x: isize, y: isize) -> bool {
    loop {
        if find_monster_orient(map, x, y, 1, 1) { break };
        if find_monster_orient(map, x, y, 1, -1) { break };
        if find_monster_orient(map, x, y, -1, 1) { break };
        if find_monster_orient(map, x, y, -1, -1) { break };

        return false
    }

    true
}

fn find_monster_orient(map: &mut Vec<Vec<char>>, x: isize, y: isize, xadd: isize, yadd: isize) -> bool {
    let mut cx = x;
    let mut cy = y;

    for o in MONSTER_OFFSETS.iter() {
        cx += o[0] * xadd;
        cy += o[1] * yadd;

        if cy < 0 || cy as usize >= map.len() {
            return false
        }

        if cx < 0 || cx as usize >= map[cy as usize].len() {
            return false
        }

        if map[cy as usize][cx as usize] != '#' {
            return false;
        }
    }

    cx = x;
    cy = y;

    for o in MONSTER_OFFSETS.iter() {
        cx += o[0] * xadd;
        cy += o[1] * yadd;

        map[cy as usize][cx as usize] = 'O'
    }

    true
}

fn count_hash(map: &Vec<Vec<char>>) -> usize {
    map.iter().map(|l| {
        l.iter().filter(|&&c| c == '#').count()
    }).sum()
}
