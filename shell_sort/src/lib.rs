fn shell_sort(arr: &mut [i32]) {
    let n = arr.len(); // 获取数组长度
    let mut gap = n / 2; // 初始化增量为数组长度的一半
    while gap > 0 {
        // 当增量大于0时，继续循环
        for i in gap..n {
            // 遍历数组中从gap开始的每一个元素
            let temp = arr[i]; // 将当前元素存储在临时变量中
            let mut j = i; // 初始化j为当前元素的下标
            while j >= gap && arr[j - gap] > temp {
                // 如果j大于等于gap且前一个元素大于当前元素，则交换两个元素的位置
                arr[j] = arr[j - gap];
                j -= gap;
            }
            arr[j] = temp; // 将临时变量中存储的值赋给arr[j]
        }
        println!("{:?}",arr);
        gap /= 2; // 缩小增量
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut vec1 = vec![4,5,3,6,9,12];
        shell_sort(&mut vec1);
        for i in 0..vec1.len() -1 {
            assert!(vec1[i] <= vec1[i + 1]);
        }
    }
}
