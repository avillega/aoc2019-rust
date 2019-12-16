use itertools::Itertools;
use std::fs;
const IMG_WIDE: usize = 25;
const IMG_TALL: usize = 6;

pub fn solve1() {
    let layers = parse_input();

    let result = layers
        .into_iter()
        .min_by_key(|layer| count_by_digit(&layer, 0))
        .map(|layer| count_by_digit(&layer, 1) * count_by_digit(&layer, 2))
        .unwrap();

    println!("{}", result)
}

pub fn solve2() {
    let mut image = vec![2u8; IMG_TALL * IMG_WIDE];

    let layers = parse_input();
    for layer in layers {
        for (idx, pixel) in layer.data.iter().enumerate() {
            if image[idx] == 2 {
                image[idx] = *pixel;
            }
        }
    }
    println!("{:?}", image);
    for row in image.chunks(IMG_WIDE) {
        for pixel in row {
            if *pixel == 0 {
                print!("█");
            } else {
                print!(" ");
            }
        }
        println!()
    }
}
struct Layer {
    data: Vec<u8>,
}

fn parse_input() -> Vec<Layer> {
    let layer_size = IMG_WIDE * IMG_TALL;
    let image = fs::read_to_string("./input/day8_1.txt")
        .unwrap()
        .lines()
        .flat_map(|line| line.chars().map(|c| c.to_digit(10).unwrap() as u8))
        .collect_vec();

    let mut layers = Vec::new();
    for chunk in image.chunks(layer_size) {
        layers.push(Layer {
            data: chunk.to_vec(),
        })
    }
    layers
}

fn count_by_digit(layer: &Layer, digit: u8) -> u64 {
    layer.data.iter().filter(|&&e| e == digit).count() as u64
}
