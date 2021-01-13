use std::io;
use std::io::Write;

fn bubble_sort(array: &mut Vec<i32>) {
    let len = array.len();
    let mut i = 1;
    while i < len {
        let mut j = 0;
        while j < (len - i) {
            if array[j] > array[j + 1] {
                let t = array[j];
                array[j] = array[j + 1];
                array[j + 1] = t;
            }
            j += 1;
        }
        i += 1;
    }
}

fn main() {
    print!("Enter size - ");
    io::stdout().flush().unwrap();
    let mut input_buf = String::new();

    io::stdin()
        .read_line(&mut input_buf)
        .expect("Error reading number");

    let size: usize = input_buf.trim().parse().expect("Error parsing input");

    let mut i = 0;

    let mut vec = Vec::new();

    println!("Enter numbers -");

    while i < size {
        let mut buf = String::new();
        io::stdin()
            .read_line(&mut buf)
            .expect("Error reading number");
        let x: i32 = buf.trim().parse().expect("Error parsing input");
        vec.push(x);
        i += 1;
    }

    bubble_sort(&mut vec);

    println!("Sorted array -");

    for x in &vec {
        print!("{} ", x);
    }
    println!();
}
