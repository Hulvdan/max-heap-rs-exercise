#[derive(Debug)]
pub struct MaxHeap<T: PartialOrd> {
    vec: Vec<T>,
}


impl<T: PartialOrd> MaxHeap<T> {
    pub fn new() -> MaxHeap<T> {
        MaxHeap { vec: Vec::<T>::new() }
    }

    pub fn from_vec(vec: Vec<T>) -> MaxHeap<T> {
        let mut heap = MaxHeap { vec };

        heap.heapify();
        heap
    }

    pub fn len(&self) -> usize {
        self.vec.len()
    }

    pub fn push(&mut self, value: T) {
        self.vec.push(value);
        self.heapify_up_to_root();
    }

    pub fn pop(&mut self) -> T {
        assert_ne!(self.vec.len(), 0);

        let last_element_index = self.vec.len() - 1;
        self.vec.swap(0, last_element_index);

        let val = self.vec.pop();

        self.heapify_down_to_leafs(0);

        val.unwrap()
    }

    pub fn get(&self) -> &T {
        assert_ne!(self.vec.len(), 0);

        &self.vec[0]
    }

    fn heapify(&mut self) {
        let count_of_nodes_that_have_children = self.vec.len() / 2;

        for i in (0..count_of_nodes_that_have_children).rev() {
            self.heapify_down_to_leafs(i);
        }
    }

    fn heapify_up_to_root(&mut self) {
        let insertion_index = self.vec.len() - 1;

        let mut i = insertion_index.clone();
        while i > 0 {
            let parent_index = Self::parent_index(i);
            self.swap_if_needed(parent_index, i);

            i = parent_index;
        }
    }

    #[inline(always)]
    fn heapify_down_to_leafs(&mut self, node_index: usize) {
        let mut parent_index = node_index;

        loop {
            let left = Self::left_child_index(parent_index);
            let right = Self::right_child_index(parent_index);

            let mut largest_child_index: usize = parent_index;
            if left < self.vec.len() && self.vec[left] > self.vec[largest_child_index] {
                largest_child_index = left;
            }
            if right < self.vec.len() && self.vec[right] > self.vec[largest_child_index] {
                largest_child_index = right;
            }

            if parent_index == largest_child_index {
                break;
            }

            self.vec.swap(largest_child_index, parent_index);
            parent_index = largest_child_index;
        }
    }

    #[inline(always)]
    fn parent_index(i: usize) -> usize {
        (i - 1) / 2
    }

    #[inline(always)]
    fn right_child_index(i: usize) -> usize {
        2 * i + 2
    }

    #[inline(always)]
    fn left_child_index(i: usize) -> usize {
        2 * i + 1
    }

    #[inline(always)]
    fn swap_if_needed(&mut self, parent_index: usize, child_index: usize) {
        if child_index >= self.vec.len() {
            return;
        }

        if self.vec[parent_index] < self.vec[child_index] {
            self.vec.swap(parent_index, child_index);
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::MaxHeap;

    #[test]
    fn test_maxheap() {
        let all_numbers = vec![
            vec![1, 4, 5, 2, 3, 6, 7, 8, 9],
            vec![18, 59, 8, 90, 41, 28, 41, 14, 16, 4, 0, 57, 51, 67, 0, 10, 43, 38, 81],
            vec![14, 98, 81, 64, 59, 16, 77, 19, 28, 84, 13, 4, 85, 16, 49, 57, 11, 2, 26, 24],
            vec![86, 73, 89, 36, 61, 5, 2, 57, 48, 59, 100, 66, 90, 72, 92, 81, 75, 48, 96, 67],
            vec![24, 45, 3, 80, 21, 76, 57, 93, 77, 79, 30, 41, 17, 86, 17, 63, 36, 6, 42, 48],
            vec![64, 16, 94, 25, 82, 72, 34, 75, 76, 95, 88, 9, 34, 95, 51, 70, 76, 88, 87, 68],
        ];

        let mut all_results = Vec::<Vec<i32>>::new();
        for numbers in all_numbers.iter() {
            let mut results = Vec::new();

            let mut heap = MaxHeap::from_vec(numbers.clone());
            for _ in 0..numbers.len() {
                results.push(heap.pop().clone());
            }

            all_results.push(results);
        }

        assert_eq!(
            all_results,
            vec![
                vec![9, 8, 7, 6, 5, 4, 3, 2, 1],
                vec![90, 81, 67, 59, 57, 51, 43, 41, 41, 38, 28, 18, 16, 14, 10, 8, 4, 0, 0],
                vec![98, 85, 84, 81, 77, 64, 59, 57, 49, 28, 26, 24, 19, 16, 16, 14, 13, 11, 4, 2],
                vec![100, 96, 92, 90, 89, 86, 81, 75, 73, 72, 67, 66, 61, 59, 57, 48, 48, 36, 5, 2],
                vec![93, 86, 80, 79, 77, 76, 63, 57, 48, 45, 42, 41, 36, 30, 24, 21, 17, 17, 6, 3],
                vec![95, 95, 94, 88, 88, 87, 82, 76, 76, 75, 72, 70, 68, 64, 51, 34, 34, 25, 16, 9],
            ]
        );
    }
}
