fn main() {
    println!("Hello, world!");




    let hello = "Здравствуйте";
    let s = &hello[0..4]; // s is now “Зд”
    println!("{s}");

    let hello = "اًلًسلام عليكم";
    let s = &hello[0..4]; // s is now “Зд”
    println!("{s}");
}
