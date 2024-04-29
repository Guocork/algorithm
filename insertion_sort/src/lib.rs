
/**
 *  插入排序
 */
fn insertion_sort<T: Ord + std::fmt::Debug + Copy>(arr: &mut [T]) {
    for i in 1..arr.len() {
        let (mut p, v) = (i, arr[i]);
        while p > 0 && arr[p - 1] > v {
            arr[p] = arr[p -1];
            p -= 1;
        }
        arr[p] = v;
    }
}

fn insertion_sort2<T>(arr: &[T]) -> Vec<T> 
where 
    T: Ord + Clone + std::fmt::Debug
{
    let mut res = Vec::with_capacity(arr.len());
    for item in arr.iter().cloned() {
        let n_len = res.len();
        for i in 0..=n_len {
            if i == n_len || res[i] > item {
                res.insert(i, item);
                println!("{:?}",res);
                break;
            }
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut vec1 = vec![4,5,3,6,9,12];
        insertion_sort(&mut vec1);
        for i in 0..vec1.len() -1 {
            assert!(vec1[i] <= vec1[i + 1]);
        }
    }

    #[test]
    fn insertion() {
        let vec = vec![4,5,6,3,2,1,];
        let res = insertion_sort2(&vec);
        assert_eq!(res,vec![1,2,3,4,5,6]);
    }
}
