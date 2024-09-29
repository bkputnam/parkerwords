use std::{array, collections::HashSet};

use crate::{
    filterable_linked_list::FilterableLinkedList, read_words::read_words,
};

pub type SolutionIndices = [usize; 5];
pub type Solution<'a> = [&'a str; 5];

pub struct GlobalData {
    pub words: Vec<String>,
    pub word_bits: FilterableLinkedList<u32>,
    pub filters: [FilterableLinkedList<usize>; 26],
}

impl GlobalData {
    pub fn new() -> GlobalData {
        let mut words: Vec<String> = Vec::new();
        let mut word_bits: Vec<u32> = Vec::new();
        let mut seen_words: HashSet<u32> = HashSet::new();
        let mut filters: [Vec<usize>; 26] =
            array::from_fn(|_: usize| Vec::new());
        for word in read_words().into_iter() {
            let bits = word_to_bits(&word);
            if bits.count_ones() == 5 && !seen_words.contains(&bits) {
                seen_words.insert(bits);
                words.push(word);
                word_bits.push(bits);

                let word_index = word_bits.len() - 1;
                for bit_index in bit_indices(bits) {
                    filters[bit_index].push(word_index)
                }
            }
        }

        GlobalData {
            words,
            word_bits: FilterableLinkedList::new(word_bits),
            filters: filters.map(|filter| FilterableLinkedList::new(filter)),
        }
    }

    pub fn filter<F: Fn(u32, usize) -> bool>(&mut self, check_item: F) {
        self.word_bits.filter(&check_item);
        for filter in self.filters.iter_mut() {
            filter.filter(|index, _| {
                let bits = self.word_bits.get_at_unfiltered_index(index);
                check_item(bits, index)
            })
        }
    }

    pub fn undo_last_filter(&mut self) {
        self.word_bits.undo_last_filter();
        for filter in self.filters.iter_mut() {
            filter.undo_last_filter();
        }
    }

    pub fn solution_indicies_to_str<'a>(
        &'a self,
        indicies: &SolutionIndices,
    ) -> Solution<'a> {
        indicies.map(|index| self.words[index].as_str())
    }
}

fn word_to_bits(word: &str) -> u32 {
    let a_byte: u8 = 'A' as u8;
    let mut result: u32 = 0;

    // Assume that word only contains uppercase ASCII characters
    for byte in word.as_bytes().iter() {
        let letter_index = byte - a_byte;
        result |= 1u32 << letter_index;
    }
    result
}

fn bit_indices(bits: u32) -> Vec<usize> {
    let mut result: Vec<usize> = Vec::new();
    let mut bits_copy = bits;
    while bits_copy != 0 {
        let index = bits_copy.trailing_zeros();
        bits_copy ^= 1 << index;
        result.push(index as usize);
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::global_data::word_to_bits;

    use super::bit_indices;

    #[test]
    fn test_word_to_bits_1() {
        assert_eq!(word_to_bits("A"), 1);
    }

    #[test]
    fn test_word_to_bits_2() {
        assert_eq!(word_to_bits("B"), 2);
    }

    #[test]
    fn test_word_to_bits_3() {
        assert_eq!(word_to_bits("ABCDE"), 31);
    }

    #[test]
    fn test_bit_indicies() {
        let bits: u32 = 0b10010111;
        let bit_is = bit_indices(bits);
        assert_eq!(bit_is.len(), 5);
        assert_eq!(bit_is[0], 0);
        assert_eq!(bit_is[1], 1);
        assert_eq!(bit_is[2], 2);
        assert_eq!(bit_is[3], 4);
        assert_eq!(bit_is[4], 7);
    }
}
