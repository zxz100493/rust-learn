// struct Point<T> {
//     x: T,
//     y: T,
// }

// fn main() {
//     let boolean = Point { x: true, y: false };
//     let integer = Point { x: 1, y: 9 };
//     let float = Point { x: 1.7, y: 4.3 };
//     let string_slice = Point { x: "high", y: "low" };
// }

// struct Point<T, U> {
//     x: T,
//     y: U,
// }

// fn main() {
//     let integer_and_boolean = Point { x: 5, y: false };
//     let float_and_string = Point { x: 1.0, y: "hey" };
//     let integer_and_float = Point { x: 5, y: 4.0 };
//     let both_integer = Point { x: 10, y: 30 };
//     let both_boolean = Point { x: true, y: true };
// }

/* trait Area {
    fn area(&self) -> f64;
}

struct Circle {
    radius: f64,
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Area for Circle {
    fn area(&self) -> f64 {
        use std::f64::consts::PI;
        PI * self.radius.powf(2.0)
    }
}

impl Area for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

fn main() {
    let circle = Circle { radius: 5.0 };
    let rectangle = Rectangle {
        width: 10.0,
        height: 20.0,
    };

    println!("Circle area: {}", circle.area());
    println!("Rectangle area: {}", rectangle.area());
} */
/* #[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

use std::fmt;

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn main() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 4, y: -3 };

    if p1 == p2 {
        // can't compare two Point values!
        println!("equal!");
    } else {
        println!("not equal!");
    }

    println!("{}", p1); // can't print using the '{}' format specifier!
    println!("{:?}", p1); //  can't print using the '{:?}' format specifier!
}
 */
/*
 #![allow(dead_code, unused_variables)]

 trait AsJson {
     fn as_json(&self) -> String;
 }

 fn send_data_as_json(value: &impl AsJson) {
     println!("Sending JSON data to server...");
     println!("-> {}", value.as_json());
     println!("Done!\n");
 }

 struct Person {
     name: String,
     age: u8,
     favorite_fruit: String,
 }

 struct Dog {
     name: String,
     color: String,
     likes_petting: bool,
 }

impl AsJson for Cat {
    fn as_json(&self) -> String {
        format!(
            r#"{{ "type": "cat", "name": "{}", "sharpClaws": {} }}"#,
            self.name, self.sharp_claws
        )
    }
}

 impl AsJson for Person {
     fn as_json(&self) -> String {
         format!(
             r#"{{ "type": "person", "name": "{}", "age": {}, "favoriteFruit": "{}" }}"#,
             self.name, self.age, self.favorite_fruit
         )
     }
 }

 impl AsJson for Dog {
     fn as_json(&self) -> String {
         format!(
             r#"{{ "type": "dog", "name": "{}", "color": "{}", "likesPetting": {} }}"#,
             self.name, self.color, self.likes_petting
         )
     }
 }

 struct Cat {
     name: String,
     sharp_claws: bool,
 }

 fn main() {
     let laura = Person {
         name: String::from("Laura"),
         age: 31,
         favorite_fruit: String::from("apples"),
     };

     let fido = Dog {
         name: String::from("Fido"),
         color: String::from("Black"),
         likes_petting: true,
     };

     let kitty = Cat {
         name: String::from("Kitty"),
         sharp_claws: false,
     };

     send_data_as_json(&laura);
     send_data_as_json(&fido);

     // The Cat type does not implement the trait AsJson.
     send_data_as_json(&kitty) // uncomment this line to see the compiler error.
 }
  */

/* struct Container<T> {
    value: T,
}

impl<T> Container<T> {
    pub fn new(value: T) -> Self {
        Container { value }
    }
}

fn main() {
    assert_eq!(Container::new(42).value, 42);
    assert_eq!(Container::new(3.14).value, 3.14);
    assert_eq!(Container::new("Foo").value, "Foo");
    assert_eq!(
        Container::new(String::from("Bar")).value,
        String::from("Bar")
    );
    assert_eq!(Container::new(true).value, true);
    assert_eq!(Container::new(-12).value, -12);
    assert_eq!(Container::new(Some("text")).value, Some("text"));
} */

struct Groups<T> {
    inner: Vec<T>,
}

impl<T> Groups<T> {
    fn new(inner: Vec<T>) -> Self {
        Groups { inner }
    }
}

impl<T: PartialEq> Iterator for Groups<T> {
    type Item = Vec<T>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.inner.is_empty() {
            return None;
        }
        let mut cursor = 1;
        let first = &self.inner[0];
        for element in &self.inner[1..] {
            if element == first {
                cursor += 1;
            } else {
                break;
            }
        }
        let items = self.inner.drain(0..cursor).collect();
        Some(items)
    }
    // TODO: Write the rest of this implementation.
}

fn main() {
    let data = vec![4, 1, 1, 2, 1, 3, 3, -2, -2, -2, 5, 5];
    // groups:     |->|---->|->|->|--->|----------->|--->|
    assert_eq!(
        Groups::new(data).into_iter().collect::<Vec<Vec<_>>>(),
        vec![
            vec![4],
            vec![1, 1],
            vec![2],
            vec![1],
            vec![3, 3],
            vec![-2, -2, -2],
            vec![5, 5],
        ]
    );

    let data2 = vec![1, 2, 2, 1, 1, 2, 2, 3, 4, 4, 3];
    // groups:      |->|---->|---->|----|->|----->|->|
    assert_eq!(
        Groups::new(data2).into_iter().collect::<Vec<Vec<_>>>(),
        vec![
            vec![1],
            vec![2, 2],
            vec![1, 1],
            vec![2, 2],
            vec![3],
            vec![4, 4],
            vec![3],
        ]
    )
}
