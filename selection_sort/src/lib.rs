/**
 * 选择排序
 */
fn selection_sort<T: Ord + std::fmt::Debug>(arr: &mut [T]) {
    let len = arr.len();
    for left_index in 0..len {
        let mut smallest_index = left_index;
        for right_index in (left_index + 1)..len {
            if arr[right_index] < arr[smallest_index] {
                smallest_index = right_index;
            }
        }
        arr.swap(smallest_index, left_index);   // 注意交换的位置
        println!("{:?}",arr);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut vec1 = vec![4,5,3,6,9,12];
        selection_sort(&mut vec1);
        for i in 0..vec1.len() -1 {
            assert!(vec1[i] <= vec1[i + 1]);
        }
    }
}
