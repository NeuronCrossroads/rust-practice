
pub fn display_vec(v: &Vec<i32>) {
    for i in 0..v.len() {
        println!("Idx: {index} = {element}",index=i,element=v[i]);
    }
}