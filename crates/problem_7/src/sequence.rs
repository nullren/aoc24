pub struct SequenceIterator<'a, T> {
    elements: &'a [T],
    indices: Vec<usize>,
    done: bool,
}

impl<'a, T: Clone> SequenceIterator<'a, T> {
    pub fn new(elements: &'a [T], m: usize) -> Self {
        // If there are no elements but m > 0, we can't produce any sequences.
        let done = elements.is_empty() && m > 0;
        Self {
            elements,
            indices: vec![0; m],
            done,
        }
    }
}

impl<'a, T: Clone> Iterator for SequenceIterator<'a, T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        // Construct the current sequence
        let sequence = self
            .indices
            .iter()
            .map(|&i| self.elements[i].clone())
            .collect();

        // Now increment indices like a base-N counter
        let n = self.elements.len();
        if n == 0 {
            // No elements means only one sequence (empty if m=0, otherwise none)
            self.done = true;
        } else {
            let m = self.indices.len();
            let mut pos = m;

            loop {
                if pos == 0 {
                    // We overflowed all positions
                    self.done = true;
                    break;
                }
                pos -= 1;
                self.indices[pos] += 1;
                if self.indices[pos] < n {
                    // Successfully incremented without overflow
                    break;
                }
                // Overflow at this position, reset to 0 and carry to previous
                self.indices[pos] = 0;
            }
        }

        Some(sequence)
    }
}
