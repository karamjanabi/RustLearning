fn main() {
    let mut v1 = vec![1, 2, 3];
    let v2 = &v1;
    v1.push(4);
    let v3 = &v1;
    println!("{:?}", v2);
    println!("{:?}", v3);
}