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


    {
        println!("Chapter 3.5: Control Flow\n");

        let mut counter = 0;

        let result = loop {
            println!("counter = {}", counter);
            counter += 1;

            if counter == 5 {
                break counter * 2;
            }
        };

        println!("The result is {}\n", result);
    }

    println!("Chapter 4.1: Ownership\n");

    {
        // after s1 moves into s2, it is no longer valid
        // let s1 = String::from("hello");
        // let s2 = s1;                // now s1 is no longer valid

        // println!("{}, world!", s1); // error
    }

    {
        // this works
        let s1 = "Michael";
        let s2 = s1;
        println!("s1 = {}, s2 = {}\n", s1, s2);
    }

    {
        // this works
        let a1 = [1, 2, 3];
        let a2 = a1;
        println!("a1 = {:?}, a2 = {:?}\n", a1, a2);
    }

}
