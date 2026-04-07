use vec_o1::VecO1;



fn main() {
    let mut container = VecO1::<_,u16>::new();
    _ = container.push(String::from("A"));
    _ = container.push(String::from("b"));
    _ = container.push(String::from("c"));
    _ = container.push(String::from("d"));
    _ = container.push(String::from("e"));
    _ = container.push(String::from("f"));
    container.remove(2);

    container.print();
}
