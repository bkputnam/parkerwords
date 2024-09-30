use crate::read_words::read_words;
use std::collections::HashMap;

pub type SolutionBits = [u32; 5];
pub type Solution<'a> = [&'a str; 5];

pub struct GlobalData {
    pub word_bits: Vec<u32>,
    pub words_by_bits: HashMap<u32, Vec<String>>,
}

impl GlobalData {
    pub fn new() -> GlobalData {
        let mut word_bits: Vec<u32> = vec![];
        let mut words_by_bits: HashMap<u32, Vec<String>> = HashMap::new();
        for word in read_words().into_iter() {
            let bits = word_to_bits(&word);
            if bits.count_ones() != 5 {
                continue;
            }
            if let Some(words) = words_by_bits.get_mut(&bits) {
                words.push(word);
            } else {
                word_bits.push(bits);
                words_by_bits.insert(bits, vec![word]);
            }
        }

        GlobalData {
            word_bits,
            words_by_bits,
        }
    }

    pub fn solution_indicies_to_strs<'a>(
        &'a self,
        bits: &SolutionBits,
    ) -> Vec<Solution<'a>> {
        // let mut result: Vec<Solution<'a>> = vec![];
        // for word_0 in self.words_by_bits.get(&indicies[0]).unwrap() {
        //     for word_1 in self.words_by_bits.get(&indicies[1]).unwrap() {
        //         for word_2 in self.words_by_bits.get(&indicies[2]).unwrap() {
        //             for word_3 in self.words_by_bits.get(&indicies[3]).unwrap()
        //             {
        //                 for word_4 in
        //                     self.words_by_bits.get(&indicies[4]).unwrap()
        //                 {
        //                     result
        //                         .push([word_0, word_1, word_2, word_3, word_4]);
        //                 }
        //             }
        //         }
        //     }
        // }
        // result

        let lone_result = bits.map(|word_bits| {
            self.words_by_bits.get(&word_bits).unwrap()[0].as_ref()
        });
        vec![lone_result]
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
}
