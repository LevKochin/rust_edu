use std::io::stdin;
fn main() {
    println!("Вычисление последовательности Фибоначчи");
    'program: loop {
        println!("Введите последовательность, которую хотите высчитать");
        let mut user_input = String::new();
        stdin().read_line(&mut user_input).expect("Ошибка ввода");

        if user_input == String::from("q") {
            break 'program;
        }

        let fibonacci_numbers: i32 = match user_input.trim().parse() {
            Ok(fibonacci_number) => fibonacci_number,
            Err(_) => continue,
        };
        if fibonacci_numbers == 0 {
            println!("0");
            break;
        } else if fibonacci_numbers == 1 {
            println!("1");
            break;
        }

        let mut prev = 1;
        let mut prev2 = 0;
        let mut result = 0;
        for _ in 2..=fibonacci_numbers {
            result = prev + prev2;
            prev2 = prev;
            prev = result;
        }

        if result != 0 {
            println!("Результат - {result}");
        }
    }
}
