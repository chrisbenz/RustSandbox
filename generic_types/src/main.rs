fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let integer = Point {x: 42, y: 22};
    let float = Point {x: 12.3333, y: 4441.223};

    println!("Point 1 {:#?}", integer);
    println!("Point 2 {:#?}", float);
    println!("x = {}", integer.x());
    println!("Distance from origin = {}", float.di)

    let newPoint = PointDiff { x: 99.222, y: 22222222};


    // let largest = largest_number(&number_list);
    // println!("Result is {}", largest);

    // let number_list2 = vec![213, 2145654, 992, 111];
    // // let largest = largest_number(&number_list2);
    // println!("Largest of the second list is {}", largest);
}

// fn largest_number<T>(list: &[T]) -> T {
//     let mut largest = list[0];

//     for &item in list {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T
}

struct PointDiff<T, U> {
    x: T,
    y: U
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}