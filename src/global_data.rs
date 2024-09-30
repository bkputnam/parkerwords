use std::{collections::HashSet};

use crate::{
    filterable_linked_list::FilterableLinkedList, read_words::read_words,
};

pub type SolutionIndices = [usize; 5];
pub type Solution<'a> = [&'a str; 5];

pub struct GlobalData {
    pub words: Vec<String>,
    pub word_bits: FilterableLinkedList,
}

impl GlobalData {
    pub fn new() -> GlobalData {
        let mut words: Vec<String> = Vec::new();
        let mut word_bits: Vec<u32> = Vec::new();
        let mut seen_words: HashSet<u32> = HashSet::new();
        for word in read_words().into_iter() {
            let bits = word_to_bits(&word);
            if bits.count_ones() == 5 && !seen_words.contains(&bits) {
                seen_words.insert(bits);
                words.push(word);
                word_bits.push(bits);
            }
        }

        GlobalData {
            words,
            word_bits: FilterableLinkedList::new(word_bits),
        }
    }

    #[allow(dead_code)]
    pub fn filter_data<F: Fn(u32, usize) -> bool>(&mut self, check_item: F) {
        self.word_bits.filter(&check_item);
    }

    #[allow(dead_code)]
    pub fn filter_bkp(&mut self, min_index: usize, word_bits: u32) {
        self.word_bits.filter_bkp(min_index, word_bits);
    }

    pub fn undo_last_filter(&mut self) {
        self.word_bits.undo_last_filter();
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
