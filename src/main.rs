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
        // Integer types default to i32: this type is generally the fastest, even on 64-bit systems.
        // The primary situation in which you’d use isize or usize is when indexing some sort of collection.

        let a = 123_456; // with a digit separator
        let b = 0xff; // hex
        let c = 0b1111_0000; // binary
        let d = b'A'; // character
        let e = 57u8; // with a type suffix
        println!("{} {} {} {} {}\n", a, b, c, d, e);
    }


    {
        // character literals

        let c = 'z';
        let z = 'ℤ';
        let heart_eyed_cat = '😻';
        println!("{} {} {}\n", c, z, heart_eyed_cat);
    }

    {
        // tuples

        let tup = (500, 6.4, 1);
        println!("{} {} {}", tup.0, tup.1, tup.2);

        let (x, y, z) = tup; // destructuring
        println!("{} {} {}\n", x, y, z);
    }

    {
        // Arrays are useful when you want your data allocated on the stack rather than the heap

        let a: [i32; 5] = [1, 2, 3, 4, 5];
        let first = a[0];
        let second = a[1];
        println!("a[0] = {}, a[1] = {}", first, second);

        let b = [3; 5]; // same as [3, 3, 3, 3, 3]
        println!("{:?}\n", b);
    }


    println!("Chapter : \n");
}
