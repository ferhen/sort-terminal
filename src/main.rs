const BLOCK: &str = "█";
const EMPTY: &str = " ";

fn main() {
    let mut list: Vec<u32> = vec![5, 2, 16, 19, 17, 6, 10, 15, 9, 7, 1, 4, 8, 12, 13, 3, 11, 14, 18, 20];
    bubble_sort(&mut list);
    println!("{:?}", list);
}

fn bubble_sort(list: &mut Vec<u32>) {
    let mut swapped = true;

    let mut x: i32 = -1;

    while swapped {
        swapped = false;
        x = x + 1;
        for i in 1..(list.len() - x as usize) {
            if list[i - 1] > list[i] {
                print_list(list);
                list.swap(i - 1, i);
                swapped = true
            }
        }
    }
    
    print_list(list);
}

fn print_list(list: &Vec<u32>) {
    let mut cloned_list = list.clone();

    let n_lines = *list.iter().max().unwrap();

    let mut result: String = String::new();

    for _ in 0..(n_lines + 1) {
        let mut line_string = String::new();

        for column in &mut cloned_list {
            if *column > 0 {
                line_string += BLOCK;
                *column -= 1;
            }
            else {
                line_string += EMPTY;
            }
        }

        line_string += "\n";

        result.insert_str(0, line_string.to_owned().as_str());
    }

    std::thread::sleep(std::time::Duration::from_millis(50));
    clear_terminal();
    println!("{}", result);
}

fn clear_terminal() {
    print!("{}[2J", 27 as char);
}