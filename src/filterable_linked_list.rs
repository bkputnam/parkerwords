#[derive(Debug)]
pub struct FilterableLinkedList {
    pub items: Vec<u32>,
    pub first_index: Option<usize>,
    last_index: Option<usize>,
    pub next_indices: Vec<Option<usize>>,
    prev_indices: Vec<Option<usize>>,
    undo_frames: Vec<Option<usize>>,
    len: usize,
}

impl FilterableLinkedList {
    pub fn new(items: Vec<u32>) -> FilterableLinkedList {
        let len = items.len();
        let mut next_indices: Vec<Option<usize>> = Vec::with_capacity(len);
        let mut prev_indices: Vec<Option<usize>> = Vec::with_capacity(len);
        for i in 0..len {
            next_indices.push(if i < len - 1 { Some(i + 1) } else { None });
            prev_indices.push(if i > 0 { Some(i - 1) } else { None });
        }
        let first_index = if len > 0 { Some(0) } else { None };
        let last_index = if len > 0 { Some(len - 1) } else { None };
        FilterableLinkedList {
            items,
            first_index,
            last_index,
            next_indices,
            prev_indices,
            undo_frames: Vec::with_capacity(len * 2),
            len,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    /**
     * Returns the value at the given index in the original data, without regard
     * to whether or not it's currently filtered from the list.
     */
    pub fn get_at_unfiltered_index(&self, index: usize) -> u32 {
        self.items[index]
    }

    #[allow(dead_code)]
    pub fn filter<F: Fn(u32, usize) -> bool>(&mut self, check_item: F) {
        let mut cursor = self.first_index;
        self.undo_frames.push(None);
        while let Some(cursor_index) = cursor {
            cursor = self.next_indices[cursor_index];
            if !check_item(self.items[cursor_index], cursor_index) {
                let old_prev = self.prev_indices[cursor_index];
                let old_next = self.next_indices[cursor_index];
                if let Some(old_prev_index) = old_prev {
                    self.next_indices[old_prev_index] = old_next;
                } else {
                    self.first_index = old_next;
                }
                if let Some(old_next_index) = old_next {
                    self.prev_indices[old_next_index] = old_prev;
                } else {
                    self.last_index = old_prev;
                }
                self.undo_frames.push(Some(cursor_index));
                self.len -= 1;
            }
        }
    }

    #[allow(dead_code)]
    pub fn filter_bkp(&mut self, min_index: usize, word_bits: u32) {
        let mut cursor = self.first_index;
        self.undo_frames.push(None);
        while let Some(cursor_index) = cursor {
            cursor = self.next_indices[cursor_index];
            // if !check_item(self.items[cursor_index], cursor_index) {
            if !((cursor_index > min_index) && (self.items[cursor_index] & word_bits == 0)) {
                let old_prev = self.prev_indices[cursor_index];
                let old_next = self.next_indices[cursor_index];
                if let Some(old_prev_index) = old_prev {
                    self.next_indices[old_prev_index] = old_next;
                } else {
                    self.first_index = old_next;
                }
                if let Some(old_next_index) = old_next {
                    self.prev_indices[old_next_index] = old_prev;
                } else {
                    self.last_index = old_prev;
                }
                self.undo_frames.push(Some(cursor_index));
                self.len -= 1;
            }
        }
    }

    pub fn undo_last_filter(&mut self) {
        while let Some(Some(index)) = self.undo_frames.pop() {
            let old_prev_opt = self.prev_indices[index];
            let old_next_opt = self.next_indices[index];
            if let Some(old_prev) = old_prev_opt {
                self.next_indices[old_prev] = Some(index);
            } else {
                self.first_index = Some(index);
            }
            if let Some(old_next) = old_next_opt {
                self.prev_indices[old_next] = Some(index);
            } else {
                self.last_index = Some(index);
            }
            self.len += 1;
        }
    }

    // pub fn first<'a>(&'a self) -> Option<&'a T> {
    //     match self.first_index {
    //         Some(index) => Some(&self.items[index]),
    //         None => None,
    //     }
    // }

    // pub fn iter_with_original_indices<'a>(&'a self) -> IndexIter<'a, T> {
    //     IndexIter::new(self)
    // }
}

impl<'a> IntoIterator for &'a FilterableLinkedList {
    type Item = u32;
    type IntoIter = FilterableLinkedListIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        FilterableLinkedListIterator::new(self)
    }
}

pub struct FilterableLinkedListIterator<'a> {
    source: &'a FilterableLinkedList,
    cursor: Option<usize>,
}

impl<'a> FilterableLinkedListIterator<'a> {
    pub fn new(
        source: &'a FilterableLinkedList,
    ) -> FilterableLinkedListIterator<'a> {
        FilterableLinkedListIterator {
            source,
            cursor: source.first_index,
        }
    }
}

impl<'a> Iterator for FilterableLinkedListIterator<'a> {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        match self.cursor {
            Some(index) => {
                let result = self.source.items[index];
                self.cursor = self.source.next_indices[index];
                Some(result)
            }
            None => None,
        }
    }
}

// pub struct IndexIter<'a> {
//     source: &'a FilterableLinkedList,
//     cursor: Option<usize>,
// }

// impl<'a> IndexIter<'a> {
//     pub fn new(source: &'a FilterableLinkedList) -> IndexIter<'a> {
//         IndexIter {
//             source,
//             cursor: source.first_index,
//         }
//     }
// }

// impl<'a> Iterator for IndexIter<'a> {
//     type Item = (usize, &'a T);

//     fn next(&mut self) -> Option<(usize, &'a T)> {
//         match self.cursor {
//             Some(index) => {
//                 let result = &self.source.items[index];
//                 self.cursor = self.source.next_indices[index];
//                 Some((index, result))
//             }
//             None => None,
//         }
//     }
// }

#[cfg(test)]
mod tests {
    use super::FilterableLinkedList;

    #[test]
    fn test_iter() {
        let linked_list = FilterableLinkedList::new(vec!["a", "b", "e"]);
        let mut expected = vec!["a", "b", "e"];
        expected.reverse();
        for item in &linked_list {
            assert_eq!(item, expected.pop().unwrap());
        }
    }

    #[test]
    fn test_filter() {
        let mut linked_list = FilterableLinkedList::new(vec![1, 2, 3, 4, 5, 6]);
        linked_list.filter(|val| val % 2 == 0);
        assert_eq!(linked_list.len(), 3);
        let mut iter = linked_list.into_iter();
        assert_eq!(2, iter.next().unwrap());
        assert_eq!(4, iter.next().unwrap());
        assert_eq!(6, iter.next().unwrap());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn test_filter_undo() {
        let mut linked_list = FilterableLinkedList::new((1..7).collect());
        linked_list.filter(|val| val % 2 == 0);
        assert_eq!(linked_list.len(), 3);
        linked_list.undo_last_filter();
        assert_eq!(linked_list.len(), 6);
        let mut iter = linked_list.into_iter();
        assert_eq!(1, iter.next().unwrap());
        assert_eq!(2, iter.next().unwrap());
        assert_eq!(3, iter.next().unwrap());
        assert_eq!(4, iter.next().unwrap());
        assert_eq!(5, iter.next().unwrap());
        assert_eq!(6, iter.next().unwrap());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn test_filter_undo_multiple() {
        let mut linked_list = FilterableLinkedList::new((1..101).collect());
        linked_list.filter(|val| val % 2 == 0);
        assert_eq!(linked_list.len(), 50);
        linked_list.filter(|val| val % 3 == 0);
        assert_eq!(linked_list.len(), 16);
        linked_list.filter(|val| val % 4 == 0);

        let mut iter = linked_list.into_iter();
        assert_eq!(12, iter.next().unwrap());
        assert_eq!(24, iter.next().unwrap());
        assert_eq!(36, iter.next().unwrap());
        assert_eq!(48, iter.next().unwrap());
        assert_eq!(60, iter.next().unwrap());
        assert_eq!(72, iter.next().unwrap());
        assert_eq!(84, iter.next().unwrap());
        assert_eq!(96, iter.next().unwrap());
        assert_eq!(None, iter.next());

        linked_list.undo_last_filter();
        assert_eq!(linked_list.len(), 16);

        linked_list.undo_last_filter();
        assert_eq!(linked_list.len(), 50);

        linked_list.undo_last_filter();
        let mut iter2 = linked_list.into_iter();
        for i in 1..101 {
            assert_eq!(i, iter2.next().unwrap());
        }
        assert_eq!(None, iter2.next());
    }
}
