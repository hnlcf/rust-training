fn main() {}

#[test]
#[should_panic]
fn test_overflow() {
    let left = i32::MAX / 2 + 10;
    let right = i32::MAX / 2 + 10;
    let mid = (left + right) / 2;

    println!("{:?}", mid);
}
