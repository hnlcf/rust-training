pub fn merge<T>(left: &Vec<T>, right: &Vec<T>) -> Vec<T>
where
    T: Ord + Copy,
{
    let mut result = vec![];
    let mut i: usize = 0;
    let mut j: usize = 0;

    while i < left.len() && j < right.len() {
        if left[i] < right[j] {
            result.push(left[i]);
            i += 1;
        } else {
            result.push(right[j]);
            j += 1;
        }
    }

    while i < left.len() {
        result.push(left[i]);
        i += 1;
    }
    while j < right.len() {
        result.push(right[j]);
        j += 1;
    }

    result
}

pub fn merge_sort<T>(arr: &Vec<T>) -> Vec<T>
where
    T: Ord + Copy,
{
    if arr.len() == 1 {
        return arr.to_vec();
    }

    let mid = arr.len() / 2;
    let left = merge_sort(&arr[..mid].to_vec());
    let right = merge_sort(&arr[mid..].to_vec());

    merge(&left, &right)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge() {
        assert_eq!(
            merge(&vec![1, 3, 4, 5, 7, 9], &vec![2, 6, 8]),
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9]
        );
    }

    #[test]
    fn test_merge_sort() {
        assert_eq!(merge_sort(&vec![0]), vec![0]);
        assert_eq!(merge_sort(&vec![5, 3, 1, 4, 7, 9]), vec![1, 3, 4, 5, 7, 9]);
        assert_eq!(merge_sort(&vec![1, 2, 3, 4, 5]), vec![1, 2, 3, 4, 5]);
        assert_eq!(merge_sort(&vec![5, 4, 3, 2, 1]), vec![1, 2, 3, 4, 5]);
    }
}
