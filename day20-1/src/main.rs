use std::fs;
use std::io::{self, BufRead};
use std::str;
use std::cmp;
use std::collections::HashMap;

const TILE_DIM: usize = 10;

enum Edge {
    North = 0,
    East = 1,
    South = 2,
    West = 3
}

#[derive(Default)]
struct Tile {
    tile_num: u16,
    tile_data: [u16; TILE_DIM],
    edges: [u16; 4]
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tiles = load_tiles()?;

    let edges = build_edge_lookup(&tiles);

    let mut result: u64 = 1;

    for t in &tiles {
        if count_outside_edges(&t, &edges) == 2 {
            println!("Tile {} is a corner", t.tile_num);
            result *= t.tile_num as u64;
        }
    }

    println!("Product of corner tile IDs is {}", result);

    Ok(())
}

enum LoadStage {
    TileNum,
    TileData,
    Blank
}

fn load_tiles() -> Result<Vec<Tile>, Box<dyn std::error::Error>> {
    // Open the file read only
    let input = fs::File::open("../input20.txt")?;

    // Create a buffered reader on the file
    let inputbuf = io::BufReader::new(input);

    let mut tiles = Vec::new();

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

                    tiles.push(tile);

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
    tile.edges[Edge::North as usize] = calc_edge(tile.tile_data[0]);
    tile.edges[Edge::South as usize] = calc_edge(tile.tile_data[TILE_DIM - 1]);

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

    tile.edges[Edge::East as usize] = calc_edge(e);
    tile.edges[Edge::West as usize] = calc_edge(w);
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

fn build_edge_lookup(tiles: &Vec<Tile>) -> HashMap<u16, Vec<&Tile>> {
    let mut map = HashMap::new();

    let mut add = |e, t| {
        match map.get_mut(e) {
            None => {
                let mut coll = Vec::new();
                coll.push(t);
                map.insert(*e, coll);
            }
            Some(coll) => {
                coll.push(t);
            }
        };
    };

    for t in tiles {
        add(&t.edges[Edge::North as usize], t);
        add(&t.edges[Edge::East as usize], t);
        add(&t.edges[Edge::South as usize], t);
        add(&t.edges[Edge::West as usize], t);
    }

    map
}

fn count_outside_edges(tile: &Tile, edges: &HashMap<u16, Vec<&Tile>>) -> u8 {
    let mut result: u8 = 0;

    let mut calc = |edge| {
        let edge_val = tile.edges[edge as usize];

        if let Some(vec) = edges.get(&edge_val) {
            if vec.len() == 1 {
                result += 1;
            }
        }
    };

    calc(Edge::North);
    calc(Edge::East);
    calc(Edge::South);
    calc(Edge::West);

    result
}