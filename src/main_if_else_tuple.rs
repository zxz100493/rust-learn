// // fn main() {
// //     // Display the message "Hello, world!"
// //     todo!("Display the message by using the println!() macro");
// // }

// // Declare Car struct to describe vehicle with four named fields
// struct Car {
//     color: String,
//     transmission: Transmission,
//     convertible: bool,
//     mileage: u32,
// }

// #[derive(PartialEq, Debug)]
// // Declare enum for Car transmission type
// enum Transmission {
//     // todo!("Fix enum definition so code compiles");
//     Manual,
//     SemiAuto,
//     Automatic,
// }

// fn car_factory(color:String,transmission:Transmission,convertible:bool) -> Car {
//     Car {
//         color: color,
//         transmission: transmission,
//         convertible: convertible,
//         mileage: 0,
//     }
// }

// fn main() {
//     // We have orders for three new cars!
//     // We'll declare a mutable car variable and reuse it for all the cars
//     let mut car = car_factory(String::from("Red"), Transmission::Manual, false);
//     println!("Car 1 = {}, {:?} transmission, convertible: {}, mileage: {}", car.color, car.transmission, car.convertible, car.mileage);

//     car = car_factory(String::from("Silver"), Transmission::Automatic, true);
//     println!("Car 2 = {}, {:?} transmission, convertible: {}, mileage: {}", car.color, car.transmission, car.convertible, car.mileage);

//     car = car_factory(String::from("Yellow"), Transmission::SemiAuto, false);
//     println!("Car 3 = {}, {:?} transmission, convertible: {}, mileage: {}", car.color, car.transmission, car.convertible, car.mileage);
// }

// Declare array, initialize all values, compiler infers length = 7
// let days = ["Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday"];

// // Declare array, initialize all values to 0, length = 5
// let bytes = [0; 5];
// Declare vector, initialize with three values
// fn main() {
//     // let three_nums = vec![15, 3, 46];
//     // println!("Initial vector: {:?}", three_nums);

//     // // Declare vector, value = "0", length = 5
//     // let zeroes = vec![0; 5];
//     // println!("Zeroes: {:?}", zeroes);

//     // let mut fruit = Vec::new();
//     // fruit.push("Apple");
//     // fruit.push("Banana");
//     // fruit.push("Cherry");
//     // println!("Fruits: {:?}", fruit);

// }
// fn main() {
//     // Create car color array
//     let colors = ["Blue", "Green", "Red", "Silver"];
//     //  = todo!("Set the enum values: 0 = Blue, 1 = Green, 2 = Red, 3 = Silver");

//     // Declare the car type and initial values
//     let mut car: Car;
//     //  todo!("Create `car` as a `Car` struct");
//     let mut engine = Transmission::Manual;

//     // Order 3 cars, one car for each type of transmission

//     // Car order #1: New, Manual, Hard top
//     car = car_factory(String::from(colors[0]), engine, true, 0);
//     println!(
//         "Car order 1: {:?}, Hard top = {}, {:?}, {}, {} miles",
//         car.age.0, car.roof, car.motor, car.color, car.age.1
//     );

//     // Car order #2: Used, Semi-automatic, Convertible
//     engine = Transmission::SemiAuto;
//     car = car_factory(String::from(colors[1]), engine, false, 100);
//     println!(
//         "Car order 2: {:?}, Hard top = {}, {:?}, {}, {} miles",
//         car.age.0, car.roof, car.motor, car.color, car.age.1
//     );

//     // Car order #3: Used, Automatic, Hard top
//     engine = Transmission::Automatic;
//     car = car_factory(String::from(colors[2]), engine, true, 200);
//     println!(
//         "Car order 3: {:?}, Hard top = {}, {:?}, {}, {} miles",
//         car.age.0, car.roof, car.motor, car.color, car.age.1
//     );
// }

// #[derive(PartialEq, Debug)]
// // Declare Car struct to describe vehicle with four named fields
// struct Car {
//     color: String,
//     motor: Transmission,
//     roof: bool,
//     age: (Age, u32), // todo!("Move `mileage: u32` field into `age` field - a tuple with two fields: an `Age` enum, u32");
// }

// #[derive(PartialEq, Debug)]
// // Declare enum for Car transmission type
// enum Transmission {
//     Manual,
//     SemiAuto,
//     Automatic,
// }
// #[derive(PartialEq, Debug)]
// enum Age {
//     New,
//     Used,
// }

// fn car_quality(miles: u32) -> (Age, u32) {
//     let quality = (Age::New, miles);
//     // Return the completed tuple to the caller
//     quality
// }

// // Build a new "Car" using the values of four input arguments
// // - color (String)
// // - motor (Transmission enum)
// // - roof (boolean, true if the car has a hard top roof)
// // - miles (u32)
// // Call the car_quality(miles) function to get the car age
// // Return an instance of a "Car" struct with the arrow `->` syntax
// fn car_factory(color: String, motor: Transmission, roof: bool, miles: u32) -> Car {
//     // Create a new "Car" instance as requested
//     // - Bind first three fields to values of input arguments
//     // - "age" field calls "car_quality" function with "miles" input argument
//     Car {
//         color: color,
//         motor: motor,
//         roof: roof,
//         age: car_quality(miles), // todo!("Replace `mileage: miles` with `age` tuple field, call `car_quality()` with `miles` as input argument");
//     }
// }
fn main() {
    // if 1==2 {
    //     println!("True, the numbers are equal."); //
    // } else {
    //     println!("False, the numbers are not equal.");
    // }

    // let formal = true;
    // let greeting = if formal {
    //     "Good day to you."
    // } else {
    //     "Hey!"
    // };
    // println!("{}", greeting)

    let num = 500; // num variable can be set at some point in the program
    let out_of_range: bool;
    if num < 0 {
        out_of_range = true;
    } else if num == 0 {
        out_of_range = true;
    } else if num > 512 {
        out_of_range = true;
    } else {
        out_of_range = false;
    }
}
