extern crate quickcheck;

// Selection sort function - given a Vector of i32's, will
// sort in ascending order if asc = true, else descending

fn selection_sort(v: &mut Vec<i32>, asc: bool) {
    
    // if 0 or 1 elements, can just return
    if v.len() < 2 {
        return
    }

    let mut min_index: usize;
    let mut min_val: i32;

    for j in 0..v.len() {
        // println!("Checking position {}", j);
        min_index = j;
        min_val = v[j];
        for (k, &i) in v.iter().enumerate().skip((j + 1)) {
        // for k in (j+1)..v.len() {
            // println!("\t{} - Comparing {} & {}", k, min_val, a[k]);
            if asc {
                if i < min_val {
                    // println!("\tSetting min_val to {}", a[k]);
                    min_val = i;
                    min_index = k;
                }
            } else {
                if v[k] > min_val {
                    // println!("\tSetting min_val to {}", a[k]);
                    min_val = i;
                    min_index = k;
                }
                
            }
        }
        // println!("{} < {} ?", v[min_index], min_val);
        // println!("Swapping locs {} & {}", j, min_index);
        v.swap(j, min_index)

    }
}


#[cfg(test)]
mod tests {

    // Get access to the fabs function
    
    use super::selection_sort;

    // QuickCheck imports
    
    use quickcheck::quickcheck;
    use quickcheck::TestResult;

    
    // Check that the last value in an ascending sort is not smaller than
    // any other value in the vector

    #[test]
    fn test_selection_sort_asc_last_element() {
        // These muts in the test fn signatures *are* needed and are fine since you're taking ownership of v
        fn prop_last_element_not_smaller(mut v: Vec<i32>) -> TestResult {
            if v.len() < 2 {
                TestResult::discard()
            } else {
                selection_sort(&mut v, true);
                let last = v.len() - 1;
                let last_elem = v[last];
                for j in 0..(last - 1) {
                    if v[j] > last_elem {
                        return TestResult::from_bool(false);
                    }
                }
                TestResult::from_bool(true)
            }
        }
        quickcheck(prop_last_element_not_smaller as fn(Vec<i32>) -> TestResult);
    }

    // A sorted array should always have the same number of elements

    #[test]
    fn test_selection_sort_same_num_elements() {
        
        fn prop_same_num_elems(mut v: Vec<i32>) -> bool {
            let orig_num = v.len();
            selection_sort(&mut v, true);
            orig_num == v.len()
        }
        
        quickcheck(prop_same_num_elems as fn(Vec<i32>) -> bool);
    }

    // Sorting an array twice should return the same result as doing it
    // once [idempotency]

    #[test]
    fn test_selection_sort_idempotent() {

        fn prop_idempotent(mut v: Vec<i32>) -> bool {
            selection_sort(&mut v, true);
            let once = v.clone();
            let mut twice = once.clone();
            selection_sort(&mut twice, true);
            
            for j in 0..v.len() {
                if once[j] != twice[j] {
                    return false
                }
            }
            true
                
        }
        
        quickcheck(prop_idempotent as fn(Vec<i32>) -> bool);
    }

}
