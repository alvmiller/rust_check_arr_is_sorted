pub fn is_sorted<T>(arr: &[T]) -> bool
where
    T: Ord,
{
    if arr.is_empty() {
        return true;
    }
    if arr.len() == 1 {
        return true;
    }

    arr.windows(2).all(|w| w[0] <= w[1])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_sorted() {
        let v = vec![3, 5, 1, 3, 2];
        assert!(!is_sorted(&v));
    }

    #[test]
    fn test_is_sorted_presorted() {
        let v = vec![1, 2, 3, 4, 5, 6];
        assert!(is_sorted(&v));
    }

    #[test]
    fn test_is_sorted_empty() {
        let v: Vec<usize> = vec![];
        assert!(is_sorted(&v));
    }
    
    #[test]
    fn test_is_sorted_one_element() {
        let v: Vec<usize> = vec![2];
        assert!(is_sorted(&v));
    }
    
    #[test]
    fn test_is_sorted_two_elements() {
        let v: Vec<usize> = vec![2, 1];
        assert!(!is_sorted(&v));
    }
    
    #[test]
    fn test_is_sorted_three_elements() {
        let v: Vec<usize> = vec![3, 2, 1];
        assert!(!is_sorted(&v));
    }
}

fn main() {
    let v = vec![4, 3, 3, 2, 1, 8, 10];
    println!("Array: {:?}", v);
    if is_sorted(&v) {
        println!("Array is sorted");
    } else {
        println!("Array is not sorted");
    }
}
