use std::iter::FromIterator;

/// TODO!
#[derive(Debug, Clone)]
pub struct MultisetPermutations<I> {
    buffer: Vec<I>,
    start: bool,
    index: usize,
}

pub fn multiset_permutations<I: Iterator>(iter: I) -> MultisetPermutations<I::Item>
where
    I: Iterator,
    I::Item: Ord,
{
    let mut buffer = Vec::from_iter(iter);
    buffer.sort_unstable_by(|a, b| b.cmp(a));
    let length = buffer.len();
    MultisetPermutations {
        buffer: buffer,
        start: true,
        index: length.saturating_sub(2),
    }
}

impl<I: Copy> Iterator for MultisetPermutations<I>
where
    I: Ord,
{
    type Item = Vec<I>;

    fn next(&mut self) -> Option<Self::Item> {
        // Start iteration with buffer itself
        if self.start {
            self.start = false;
            return Some(self.buffer.clone());
        }

        // Exhausted iteration
        let has_two_next = self.index + 2 < self.buffer.len();
        if !has_two_next
            && (self.buffer.len() <= self.index + 1
                || self.buffer[0] <= self.buffer[self.index + 1])
        {
            return None;
        }

        // Determine shift index
        let shift_index = if has_two_next && self.buffer[self.index + 2] <= self.buffer[self.index]
        {
            self.index + 2
        } else {
            self.index + 1
        };

        // Prefix shift
        let shift_elem = self.buffer[shift_index];
        let mut swap_index = shift_index;
        while swap_index > 0 {
            self.buffer[swap_index] = self.buffer[swap_index - 1];
            swap_index -= 1;
        }
        self.buffer[0] = shift_elem;

        // Update index
        if self.buffer[0] < self.buffer[1] {
            self.index = 0;
        } else {
            self.index += 1;
        }

        Some(self.buffer.clone())
    }
}

#[cfg(test)]
mod tests {
    use crate::Itertools;

    #[test]
    fn test1() {
        let mut iter = vec![1, 4, 2, 1].into_iter().multiset_permutations();
        assert_eq!(iter.next(), Some(vec![4, 2, 1, 1]));
        assert_eq!(iter.next(), Some(vec![1, 4, 2, 1]));
        assert_eq!(iter.next(), Some(vec![4, 1, 2, 1]));
        assert_eq!(iter.next(), Some(vec![1, 4, 1, 2]));
        assert_eq!(iter.next(), Some(vec![1, 1, 4, 2]));
        assert_eq!(iter.next(), Some(vec![4, 1, 1, 2]));
        assert_eq!(iter.next(), Some(vec![2, 4, 1, 1]));
        assert_eq!(iter.next(), Some(vec![1, 2, 4, 1]));
        assert_eq!(iter.next(), Some(vec![2, 1, 4, 1]));
        assert_eq!(iter.next(), Some(vec![1, 2, 1, 4]));
        assert_eq!(iter.next(), Some(vec![1, 1, 2, 4]));
        assert_eq!(iter.next(), Some(vec![2, 1, 1, 4]));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test3() {
        use rand::Rng;
        use std::time::Instant;

        let n = 12;
        let mut rng = rand::thread_rng();
        let vec: Vec<i32> = (0..n).map(|_| rng.gen_range(0, 10)).collect();

        println!("{:?}", vec);

        let now = Instant::now();
        let mut permutations1 = vec
            .clone()
            .into_iter()
            .permutations(n)
            .unique()
            .collect_vec();
        println!("permutations: {:.2?}", now.elapsed());

        let now = Instant::now();
        let mut permutations2 = vec.into_iter().multiset_permutations().collect_vec();
        println!("multiset_permutations: {:.2?}", now.elapsed());

        permutations1.sort();
        permutations2.sort();
        // println!("{:?}", permutations1);

        // println!("{:?}", permutations2);

        assert_eq!(permutations1, permutations2);
    }

    #[test]
    fn test4() {
        let mut iter = vec![0, 0, 1].into_iter().multiset_permutations();
        assert_eq!(iter.next(), Some(vec![1, 0, 0]));
        assert_eq!(iter.next(), Some(vec![0, 1, 0]));
        assert_eq!(iter.next(), Some(vec![0, 0, 1]));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test5() {
        let mut iter = vec![1, 1].into_iter().multiset_permutations();
        assert_eq!(iter.next(), Some(vec![1, 1]));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test6() {
        let iter = "MISSISSIPPI".chars().multiset_permutations();
        assert_eq!(iter.count(), 34650); // 34650 = 11! / (1! * 2! * 4! * 4!)
    }

    #[test]
    fn test7() {
        let mut iter: crate::MultisetPermutations<i32> = vec![].into_iter().multiset_permutations();
        assert_eq!(iter.next(), Some(vec![]));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn timing() {
        use std::time::Instant;
        let now = Instant::now();

        let iter = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 0]
            .iter()
            .multiset_permutations();
        let count = iter.count();

        let elapsed = now.elapsed();
        println!("Elapsed: {:.2?} {count}", elapsed);
    }
}
