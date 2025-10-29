use frunk::monoid::combine_all;

fn main() {
    println!("Hello, world!");

    let v = vec![Some(1), Some(3)];
    
    let combined = combine_all(&v);
    println!("Combined result: {:#?}", combined);

    assert_eq!(combine_all(&v), Some(4));

    // Slightly more magical
    let t1 = (1, 2.5f32, String::from("hi"), Some(3));
    let t2 = (1, 2.5f32, String::from(" world"), None);
    let t3 = (1, 2.5f32, String::from(", goodbye"), Some(10));
    let tuples = vec![t1, t2, t3];

    let expected = (3, 7.5f32, String::from("hi world, goodbye"), Some(13));
    assert_eq!(combine_all(&tuples), expected);
}
