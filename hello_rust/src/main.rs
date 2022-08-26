use crate::sub::times;

mod sub; // other file import
struct User {
    user_name: String,
    age: i8,
    active: bool,
}

// enum Various {
//     DAY(i8),
//     DATE(String),
// }

fn main() {
    variable();
    println!("add: {}", add(1,2));
    coordinates(&(1, 1));
    let user = User { // structure
        user_name: String::from("test"),
        age: 10,
        active: true
    };
    println!("User Name: {}, age: {}, status: {}", user.user_name, user.age, user.active);
    println!("other file function: {}", times(1.0, 2.0));
    let mut s = String::from("this is test ");
    change(&mut s); // &mut is to borrow
    println!("{}", s);
}

#[allow(unused)]
fn variable() {
    // normal va&riable
    let mut a = 1; // mutable, default is immutable
    println!("a: {}", a);
    a = 2;
    let b = 0; // this is sentence(式はexpressionっていうらしい)
    println!("a: {a}");
    println!("{}", b+1);
    
    let (x,y,z) = (1,2,3);
    println!("x: {}, y: {}, z: {}", x, y, z); // variable expansion(parameter expansion)
    let (i,_,_) = (7,8,9); // _ is wildcard, whici is ignored
    println!("i = {}", i);

    let str_len = String::from("Hello world"); // same variable name can be reused
    let str_len = str_len.len(); // but probably re-declaration is needed due to type mismatch
    println!("str_len: {}", str_len);
    // str_len = String::from("test"); this line will be error

    let foo = 0; // shadowing
    {
        // local scope
        let foo = 100;
        println!("foo: {}", foo); // local foo is printed
    }
    println!("foo: {}", foo); // outer foo is printed

    let a = (b as i16) + 10; // as
    println!("a: {}", a);
    let bool = true;
    println!("boolean: {}", bool as i8);

    let a = [0; 5]; // each element in list is initialized as 0
    let a: [i32; 5] = [1,2,3,4,5]; // annotation says [type; the number of elements]
    let a_slice = &a[1..3];
    dbg!(a_slice);

    let a = 1..10; // sequence of number (type of this is Range)
    // dbg!(a);
    for i in a {
        print!("{} ",i);
    }
    println!();

    let s = String::from("Hello");
    let slice = &s[..];
    dbg!(slice);
    let slice = &s[..2];
    dbg!(slice);
    let slice = &s[0..=3]; // end of index is n when using =n
    dbg!(slice);

    let flag = true; // if sentence
    if flag {
        println!("flag: {}", flag);
    } else if !flag {
        println!("flag: {}", flag);
    }

    let mut counter = 0; // loop
    let result = loop {
        counter += 1;
        if counter != 10 {
            continue;
        }
        break counter * 2;
    };
    println!("result: {}", result);
    
    let mut counter = 0;  // while
    while counter != 0 {
        counter -= 1;
    }
    println!("exit from while");

    let it = [1, 2, 3, 4];
    for i in it.iter() {
        print!("{} ", i);
    }
    println!();

    struct Point(i32, i32); // tuple structure
    impl Point { // implement can add function to struct
        fn area(&self) -> f32 {
            (self.0 * self.1) as f32
        }
    }
    let origin = Point(10, 10);
    println!("x: {}, y: {},", origin.0, origin.1);
    println!("area: {}", origin.area());
    struct Color(i8, i8, i8);
    let Color (x, y, z) = Color(1, 1, 1);
    println!("x: {}, y: {}, z: {}", x, y, z);

    // let date = Various::DATE(String::from("::Sunday"));
    // let day = Various::DAY(10);

    let a = vec![1, 2, 3, 4, 5, 6];
    let _b = a.clone();

    let guess: u32 = "42".parse().expect("not a number");

    let tup: (i32, u8, bool) = (10, 0, true); // type annotationはなくてもいい
    let (x, y, z): (i32, u8, bool) = tup; // distribution
    let a = tup.0;
    if a > 0 {
        println!("This is true");
    } else {
        println!("This is false");
    }
    if a != 0 { // ここで if a とするとダメ、Rustはifの論理式にbool値しか入れられない
        println!("failure");
    }
    let a = true;
    let number = if a { 5 } else { 6 };
    let a = [0, 1, 2, 3, 4, 5];
    for i in a {
        print!("{} ", i);
    }
    println!();
    {
        let a = 0;
    } // end of scope of a

}

// each argument must have type annotation
fn add(a: i32, b: i32) -> i32 {
    return a + b
}

fn coordinates(&(a, b): &(i32, i32)) {
    println!("{}, {}", a, b);
    println!("{}, {}", a + b, a * b);
}

fn change(s: &mut String) {
    s.push_str("string")
}