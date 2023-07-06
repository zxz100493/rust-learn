// fn main() {
//     // {
//     //     let mascot = String::from("ferris");
//     //     let ferris = mascot;
//     //     println!("{}", mascot) // We'll try to use mascot after we've moved ownership of the string data from mascot to ferris.
//     // }

//     let greeting = String::from("hello");
//     let greeting_reference = &greeting; // We borrow `greeting` but the string data is still owned by `greeting`
//     println!("Greeting: {}", greeting); // We can still use `greeting`
// }

// fn print_greeting(message: &String) {
//     println!("Greeting: {}", message);
// }

// fn main() {
//     let greeting = String::from("Hello");
//     print_greeting(&greeting); // `print_greeting` takes a `&String` not an owned `String` so we borrow `greeting` with `&`
//     print_greeting(&greeting); // Since `greeting` didn't move into `print_greeting` we can use it again
// }

// fn change(message: &String) {
//     message.push_str("!"); // We try to add a "!" to the end of our message
// }

// fn main() {
//     let greeting = String::from("Hello");
//     change(&greeting);
// }

// fn main() {
//     let mut greeting = String::from("hello");
//     change(&mut greeting);
// }

// fn change(text: &mut String) {
//     text.push_str(", world");
// }

// fn main() {
//     let x;
//     {
//         let y = 42;
//         x = &y; // We store a reference to `y` in `x` but `y` is about to be dropped.
//     }
//     println!("x: {}", x); // `x` refers to `y` but `y has been dropped!
// }

// fn main() {
//     let magic1 = String::from("abracadabra!");
//     let result;
//     {
//         let magic2 = String::from("shazam!");
//         result = longest_word(&magic1, &magic2);
//     }
//     println!("The longest magic word is {}", result);
// }

// fn longest_word<'a>(x: &'a String, y: &'a String) -> &'a String {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

// #[derive(Debug)]
// struct Highlight<'document>(&'document str);

// fn main() {
//     let text = String::from("The quick brown fox jumps over the lazy dog.");
//     let fox = Highlight(&text[4..19]);
//     let dog = Highlight(&text[35..43]);
//     println!("{:?}", fox);
//     println!("{:?}", dog);
// }

// TODO: modify only this function.
fn copy_and_return<'a>(vector: &'a mut Vec<String>, value: &'a str) -> &'a String {
    vector.push(String::from(value));
    vector.get(vector.len() - 1).unwrap()
}

fn main() {
    let name1 = "Joe";
    let name2 = "Chris";
    let name3 = "Anne";

    let mut names = Vec::new();

    assert_eq!("Joe", copy_and_return(&mut names, &name1));
    assert_eq!("Chris", copy_and_return(&mut names, &name2));
    assert_eq!("Anne", copy_and_return(&mut names, &name3));

    assert_eq!(
        names,
        vec!["Joe".to_string(), "Chris".to_string(), "Anne".to_string()]
    )
}
