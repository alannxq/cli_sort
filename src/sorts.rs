use crate::bar::{Bar, output_bars};

pub fn bubble_sort(bars: &mut Vec<Bar>) {
    for j in 0..(bars.len() - 1) {
        for i in 0..(bars.len() - j - 1) {
            if bars[i].value > bars[i+1].value {
                bars.swap(i, i+1);
            }
            output_bars(&bars);
        }
    }
}

pub fn insertion_sort(bars: &mut Vec<Bar>) {
    for i in 0..(bars.len()) {
        let mut j = i;

        while j > 0 && bars[j - 1].value > bars[j].value {
            bars.swap(j-1, j);
            j -= 1;
            output_bars(&bars);            
        } 
    }
}
