
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        
    }
}
