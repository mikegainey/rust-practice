#![allow(unused_variables)]
#![allow(dead_code)]
fn main() {

    // chapter3();
    // chapter4()
    chapter5()
}

fn chapter3() {
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
}

fn chapter4() {
    println!("Chapter 4.1: Ownership\n");

    {
        // after s1 moves into s2, it is no longer valid
        // let s1 = String::from("hello");
        // let s2 = s1;                // now s1 is no longer valid

        // println!("{}, world!", s1); // error
    }

    {
        // this works
        let s1 = "Michael"; // what is the type of s1?  str or &str ? Is str copy?
        let s2 = s1;
        println!("s1 = {}, s2 = {}\n", s1, s2);
    }

    {
        // this works
        let a1 = [1, 2, 3];
        let a2 = a1;
        println!("a1 = {:?}, a2 = {:?}\n", a1, a2);
    }

    {
        println!("Chapter 4.2: References & Borrowing\n");

        let s1 = String::from("hello");

        let len = calculate_length(&s1);

        println!("The length of '{}' is {}.\n", s1, len);

        fn calculate_length(s: &String) -> usize {
            s.len()
        }
    }

    {
        // derefrencing
        let s = String::from("michael");

        let sr = &s;

        println!("sr = {}, *sr = {}", sr, *sr); // automatic dereferecing?
    }

    {
        // a *reference’s* scope starts from where it is introduced
        // and continues through the last time that reference is used.
        let mut s = String::from("hello");

        let r1 = &s; // no problem
        let r2 = &s; // no problem
        println!("{} and {}", r1, r2);
        // r1 and r2 are no longer used after this point

        let r3 = &mut s; // no problem
        println!("{}\n", r3);
    }

    // At any given time, you can have either one mutable reference or any number of immutable references.

    {
        // string slices
        let s = "string literal";
        println!("{}\n", &s[..6]); // will give an error without the &

        // my speculation:
        // s[..6] doesn't have a known size; it must be the actual characters
        // &s[..6] is the string slice: a pointer and length (which has a known size)
    }

    {
        fn first_word(s: &str) -> &str {
            let bytes = s.as_bytes();

            for (i, &item) in bytes.iter().enumerate() {
                if item == b' ' {
                    return &s[0..i];
                }
            }

            &s[..]
        }

        let my_string = String::from("hello world");

        // first_word works on slices of `String`s
        let word = first_word(&my_string[..]);
        println!("first word from string = {}", word);

        let my_string_literal = "hello world";

        // first_word works on slices of string literals
        let word = first_word(&my_string_literal[..]);
        println!("first word from string literal = {}", word);

        // Because string literals *are* string slices already,
        // this works too, without the slice syntax!
        let word = first_word(my_string_literal);
        println!("first word from string literal = {}\n", word);
    }

    {
        // error!
        // let s = "michael";
        // println!("{}", s[..3]); // doesn't have a size known at compile-time
    }
}

fn chapter5() {
    println!("\nChapter 5.1: Defining and Instantiating Structs\n");

    {
        struct User {
            username: String,
            email: String,
            sign_in_count: u64,
            active: bool,
        }

        let user1 = User {
            email: String::from("someone@example.com"),
            username: String::from("someusername123"),
            active: true,
            sign_in_count: 1,
        };

    }


}

