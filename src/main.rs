use value::value4::MyValue;

fn main() {
    println!("Hello, world!");

    let val = MyValue::from_int(5);
    println!("int to string 1");
    assert_eq!(*val.to_string(), "5".to_string());
    println!("int to string 2");
    assert_eq!(*val.to_string(), "5".to_string());
}
