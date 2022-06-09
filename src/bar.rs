use std::{time, thread};
use std::io::{stdout, Write};

const BOTTOM_LAYER: i32 = 0;
const TOP_LAYER: i32 = 10;

pub struct Bar {
    pub bar: Vec<char>,
    pub value: i32
}

fn get_bar(value: i32) -> Vec<char> {
    let bar_char = '|';
    match value {
        1 => vec![' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', bar_char],
        2 => vec![' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', bar_char, bar_char],
        3 => vec![' ', ' ', ' ', ' ', ' ', ' ', ' ', bar_char, bar_char, bar_char],
        4 => vec![' ', ' ', ' ', ' ', ' ', ' ', bar_char, bar_char, bar_char, bar_char],
        5 => vec![' ', ' ', ' ', ' ', ' ', bar_char, bar_char, bar_char, bar_char, bar_char],
        6 => vec![' ', ' ', ' ', ' ', bar_char, bar_char, bar_char, bar_char, bar_char, bar_char],
        7 => vec![' ', ' ', ' ', bar_char, bar_char, bar_char, bar_char, bar_char, bar_char, bar_char],
        8 => vec![' ', ' ', bar_char, bar_char, bar_char, bar_char, bar_char, bar_char, bar_char, bar_char],
        9 => vec![' ', bar_char, bar_char, bar_char, bar_char, bar_char, bar_char, bar_char, bar_char, bar_char],
        10 => vec![bar_char, bar_char, bar_char, bar_char, bar_char, bar_char, bar_char, bar_char, bar_char, bar_char],
        _ => vec![' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ']
    }
}

pub fn get_bars(values: &[i32]) -> Vec<Bar> {
    let mut all_bars: Vec<Bar> = Vec::new();
    for value in values {
        all_bars.push(Bar {
            bar: get_bar(*value),
            value: *value
        })
    }
    all_bars
}

pub fn output_bars(bars: &Vec<Bar>) {
    print!("{}c", 27 as char);
    stdout().flush().expect("Could not flush stdout");
    
    for layer in BOTTOM_LAYER..TOP_LAYER {
        for bar in bars {
            print!("{} ", bar.bar[layer as usize])
        }
        stdout().flush().expect("Could not flush stdout");
        println!(""); // new line for next layer
    }
    thread::sleep(time::Duration::from_millis(200));
}
