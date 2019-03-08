mod my_utils;

fn main() {
    println!("Hello, world!");
    let mut v = vec![1,2,3];
    my_utils::display_vec(&v);

    v.push(34);

    my_utils::display_vec(&v);

    v.remove(0);

    my_utils::display_vec(&v);

    let arr = [1,2,3];
    for element in arr.iter() {
        println!("{}",element);
    }
}