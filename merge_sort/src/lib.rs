fn merge_sort(arr: &mut [i32]) {
    let mid = arr.len() / 2;
    if mid == 0 {
        return;
    }

    merge_sort(&mut arr[..mid]);
    merge_sort(&mut arr[mid..]);

    let mut ret = arr.to_vec();

    merge(&arr[..mid], &arr[mid..], &mut ret[..]);

    arr.copy_from_slice(&ret);
    println!("{:?}",arr);
}

fn merge(a: &[i32],b: &[i32],ret:&mut [i32]) {
    let mut i = 0; // a
    let mut j = 0; // b
    let mut k = 0; // ret

    while i < a.len() && j < b.len() {
        if a[i] < b[j] {
            ret[k] = a[i];
            k += 1;
            i += 1;
        } else {
            ret[k] = b[j];
            k += 1;
            j += 1;
        }
    }

    if i < a.len() {
        ret[k..].copy_from_slice(&a[i..]);
    }

    if j < b.len() {
        ret[k..].copy_from_slice(&b[j..]);
    }
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut vec1 = vec![4,5,3,6,9,12];
        merge_sort(&mut vec1);
        for i in 0..vec1.len() -1 {
            assert!(vec1[i] <= vec1[i + 1]);
        }
    }
}
