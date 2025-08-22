use std::cmp::Ordering;


fn binary_search(arr: &[usize], target: usize) -> Option<usize> {

    let mut left = 0;
    let mut right = arr.len() - 1;

    while left <= right {
        let mid = left + (right - left)/2;
        match arr[mid].cmp(&target) {
            Ordering::Equal => return Some(mid),
            Ordering::Less => {
                left = mid + 1
            }
            Ordering::Greater => {
                if mid == 0 {break}
                right = mid - 1;
            }
        }
    }

    None
}


/**
 * Binary Search
 * Time Complexity: O(log n)
 * Space Complexity: O(1)
 * 
Example:
    let arr = [1, 2, 3, 4, 5];
    let target = 3;
    let result = binary_search(&arr, target);
    println!("Result: {:?}", result.unwrap());
 */