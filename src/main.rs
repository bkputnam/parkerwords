mod global_data;
mod read_words;

use global_data::{GlobalData, SolutionBits};
use rayon::prelude::*;

type Word = (usize, u32);

fn main() {
    let global_data = GlobalData::new();

    let indexed_word_bits: Vec<Word> =
        global_data.word_bits.iter().copied().enumerate().collect();

    let solution_bits: Vec<SolutionBits> = indexed_word_bits
        .par_iter()
        .flat_map(|word_0: &Word| -> Vec<SolutionBits> {
            let mut filtered_1: Vec<Word> =
                Vec::with_capacity(global_data.word_bits.len());
            let mut filtered_2: Vec<Word> =
                Vec::with_capacity(global_data.word_bits.len());
            let mut filtered_3: Vec<Word> =
                Vec::with_capacity(global_data.word_bits.len());
            let mut filtered_4: Vec<Word> =
                Vec::with_capacity(global_data.word_bits.len());

            let mut result: Vec<SolutionBits> = vec![];
            filter(word_0, &indexed_word_bits, &mut filtered_1);
            for word_1 in &filtered_1 {
                filter(word_1, &filtered_1, &mut filtered_2);
                for word_2 in &filtered_2 {
                    filter(word_2, &filtered_2, &mut filtered_3);
                    for word_3 in &filtered_3 {
                        filter(word_3, &filtered_3, &mut filtered_4);
                        for word_4 in &filtered_4 {
                            result.push([
                                word_0.1, word_1.1, word_2.1, word_3.1,
                                word_4.1,
                            ]);
                        }
                    }
                }
            }
            result
        })
        .collect();

    for solution_indices in solution_bits {
        print_solution(&solution_indices, &global_data);
    }
}

fn filter(&(index, bits): &Word, src: &Vec<Word>, dest: &mut Vec<Word>) {
    dest.clear();
    dest.extend(
        src.iter()
            .skip_while(|(index_iter, _val)| *index_iter <= index)
            .filter(|(_index, val)| *val & bits == 0),
    );
}

fn print_solution(solution_indices: &SolutionBits, global_data: &GlobalData) {
    let solutions = global_data.solution_indicies_to_strs(&solution_indices);
    for words in &solutions {
        for i in 0..words.len() {
            if i == words.len() - 1 {
                println!("{}", words[i]);
            } else {
                print!("{} ", words[i]);
            }
        }
    }
}
