fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let largest = largest_number(&number_list);
    println!("Result is {}", largest);

    let number_list2 = vec![213, 2145654, 992, 111];
    let largest = largest_number(&number_list2);
    println!("Largest of the second list is {}", largest);
}

fn largest_number<T>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
