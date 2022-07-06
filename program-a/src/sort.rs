use std::cmp::Ordering;

pub trait Sort<T> {
    fn comb_sort(&mut self);
}

pub trait SortBy<T> {
    fn comb_sort_by(&mut self, f: impl Fn(&T, &T) -> Ordering);
}

impl<T: Ord> Sort<T> for [T] {
    fn comb_sort(&mut self) {
        let shrink = 1.3;
        let mut gap = self.len();

        while gap > 1 {
            gap = (((gap as f64) / shrink) as usize).max(1);
            let mut index = 0;
            while index + gap < self.len() {
                if self[index] > self[index + gap] {
                    self.swap(index, index + gap);
                }
                index += 1;
            }
        }
    }
}

impl<T> SortBy<T> for [T] {
    fn comb_sort_by(&mut self, f: impl Fn(&T, &T) -> Ordering) {
        let shrink = 1.3;
        let mut gap = self.len();

        while gap > 1 {
            gap = (((gap as f64) / shrink) as usize).max(1);
            let mut index = 0;
            while index + gap < self.len() {
                if f(&self[index], &self[index + gap]) == Ordering::Greater {
                    self.swap(index, index + gap);
                }
                index += 1;
            }
        }
    }
}

mod tests {
    #[test]
    fn test_comb_sort() {
        use crate::sort::Sort;
        let mut v = vec![5, 4, 3, 2, 1];
        v.comb_sort();
        assert_eq!(v, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_comb_sort_by() {
        use crate::sort::SortBy;
        let mut v = vec![5, 4, 3, 2, 1];
        v.comb_sort_by(|a, b| a.cmp(b));
        assert_eq!(v, vec![1, 2, 3, 4, 5]);
    }
}
