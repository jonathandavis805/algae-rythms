fn selection(unsorted: &mut Vec<i32>) {
    for x in 0..unsorted.len() {
        let mut min_index = x;
        for y in x..unsorted.len() {
            if unsorted[y] < unsorted[min_index] {
                min_index = y
            }
        }
        unsorted.swap(x, min_index);
    }
}

#[cfg(test)]
mod tests {
    use rand::Rng;
    use crate::sorting::selection;

    #[test]
    fn test_selection_dont_move() {
        let immut_vec = vec![0, 1, 2, 3, 5, 6];
        let mut test_vec = vec![0, 1, 2, 3, 5, 6];
        selection(&mut test_vec);
        assert_eq!(immut_vec, test_vec)
    }

    #[test]
    fn test_selection() {
        let immut_vec = vec![0, 1, 2, 3, 5, 6];
        let mut test_vec = vec![0, 2, 1, 3, 5, 6];
        selection(&mut test_vec);
        assert_eq!(immut_vec, test_vec)
    }

    #[test]
    fn test_selection_dup() {
        let immut_vec = vec![0, 2, 2, 3, 5, 6];
        let mut test_vec = vec![0, 2, 2, 3, 5, 6];
        selection(&mut test_vec);
        assert_eq!(immut_vec, test_vec)
    }

    #[test]
    fn test_large_against_builtin() {
        let mut rng = rand::thread_rng();
        let length = 0..10000;
        let mut rand_vec: Vec<i32> = (length.clone()).map(|_| rng.gen_range(length.clone())).collect();

        let mut test_vec = rand_vec.clone();
        println!("builtin start: {}", chrono::Local::now().format("%H:%M:%S.%f"));
        rand_vec.sort();
        println!("builtin done: {}", chrono::Local::now().format("%H:%M:%S.%f"));
        println!("implementation start: {}", chrono::Local::now().format("%H:%M:%S.%f"));
        selection(&mut test_vec);
        println!("implementation done: {}", chrono::Local::now().format("%H:%M:%S.%f"));
        assert_eq!(rand_vec, test_vec)
    }
}