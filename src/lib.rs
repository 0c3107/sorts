pub fn add(left: usize, right: usize) -> usize {
    left + right
}

mod sorts {
    pub mod bubble_sort {

        // Sorting a vector in ascending order
        #[allow(dead_code)]
        pub fn bubble_sort<T: Ord>(arr: &mut [T]) -> &mut [T] {
            for i in 0..arr.len() {
                for j in 0..arr.len() - 1 - i {
                    if arr[j] > arr[j + 1] {
                        arr.swap(j, j + 1);
                    }
                }
            }
            arr
        }
    }
}

#[cfg(test)]
mod tests {
    use std::i64::MAX;
    use super::*;
    use rand::Rng;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
    #[test]
    fn bubble_sort_ascending_small() {
        let mut vec = vec![5, 4, 2, 3, 1];
        let mut vec_unsorted = vec.clone();
        let res = sorts::bubble_sort::bubble_sort(&mut vec_unsorted).to_owned();
        vec.sort();
        assert_eq!(res, vec);
    }
    #[test]
    fn bubble_sort_ascending_big() {
        let mut rng = rand::thread_rng();
        let size = 10_000;
        let mut vec: Vec<i64> = (0..size).map(|_| rng.gen_range(i64::MIN, i64::MAX)).collect();
        let mut vec_send = vec.clone();
        let res = sorts::bubble_sort::bubble_sort(&mut vec_send);
        vec.sort();
        assert_eq!(res, vec);
    }
}
