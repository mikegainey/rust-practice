fn main() {
    println!("\nChapter 3.1: Variables and Mutability\n");
    println!("Chapter 3.2: Data Types\n");

    {
        // converting from a string to a number
        let xs = "5";
        let x: u32 = xs.parse().expect("not a number");
        println!("x = {}\n", x);
    }

    {
        // integer literals
        let a = 123_456; // with a digit separator
        let b = 0xff; // hex
        let c = 0b1111_0000; // binary
        let d = b'A'; // character
        let e = 57u8; // with a type suffix
        println!("{} {} {} {} {}\n", a, b, c, d, e);
    }

    // Integer types default to i32: this type is generally the fastest, even on 64-bit systems.
    // The primary situation in which you’d use isize or usize is when indexing some sort of collection.

}
