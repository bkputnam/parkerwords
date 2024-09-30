mod filterable_linked_list;
mod global_data;
mod read_words;

use global_data::{GlobalData, SolutionIndices};

fn main() {
    let mut global_data = GlobalData::new();
    let mut output: Vec<SolutionIndices> = vec![];

    winnow(
        &mut global_data,
        /* currentSolution= */
        &mut [0, 0, 0, 0, 0],
        /* depth= */ 0,
        &mut output,
    );

    for solution_indices in output {
        let solution = global_data.solution_indicies_to_str(&solution_indices);
        for (index, word) in solution.iter().enumerate() {
            if index == solution.len() - 1 {
                println!("{}", word);
            } else {
                print!("{} ", word);
            }
        }
    }
}

fn print_solution(
    solution_indices: &SolutionIndices,
    global_data: &GlobalData,
) {
    let solution = global_data.solution_indicies_to_str(solution_indices);
    for (index, word) in solution.iter().enumerate() {
        if index == solution.len() - 1 {
            println!("{}", word);
        } else {
            print!("{} ", word);
        }
    }
}

fn winnow(
    global_data: &mut GlobalData,
    current_solution: &mut SolutionIndices,
    depth: usize,
    output: &mut Vec<SolutionIndices>,
) {
    let mut cursor = global_data.word_bits.first_index;
    while let Some(cursor_index) = cursor {
        let cursor_bits = global_data.word_bits.items[cursor_index];
        current_solution[depth] = cursor_index;
        cursor = global_data.word_bits.next_indices[cursor_index];
        if depth == 4 {
            output.push(current_solution.clone());
            print_solution(current_solution, global_data);
            continue;
        }
        // global_data.filter_data(|word_bits, index| {
        //     (index > cursor_index) && (word_bits & cursor_bits == 0)
        // });
        global_data.filter_bkp(cursor_index, cursor_bits);
        winnow(global_data, current_solution, depth + 1, output);
        global_data.undo_last_filter();
    }
}
