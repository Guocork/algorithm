/**
 * 冒泡排序
 */

fn bubble_sort<T: Ord + std::fmt::Debug>(arr: &mut [T]) {
    for i in 0..arr.len() {
        for j in 0..arr.len() - 1 - i {  // 最后一次不用再去比较了 以为已经是最大了
            if arr[j] > arr[j + 1] {            // 这里会有两个小坑 要记住一下 第一个  是rust中的区间是左闭右开
                arr.swap(j, j + 1);        // 另一个是在使用数组时 要注意不要让数组超出范围 即下面使用数组下标时候
                println!("{:?}",arr);           //  如果有小标+1 这种情况 一定要注意 要让上面的循环的次数 -1
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn it_works() {
        let mut vec1 = vec![4,5,3,6,9,12];
        bubble_sort(&mut vec1);
        for i in 0..vec1.len() -1 {
            assert!(vec1[i] <= vec1[i + 1]);
        }
    }
}
