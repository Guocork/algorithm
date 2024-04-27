/**
 * 二位数组的打印 时间复杂度n**2
 */


 fn print_array(vec: Vec<[i32;2]>) {
    for items in vec.iter() {
        for item in items {
            print!("{} ",item);
        }
    }
}

fn main() {
    let v1: Vec<[i32; 2]> = vec![[1,2],[3,5],[8,0]];

    print_array(v1);
}
