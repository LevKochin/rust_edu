use std::io::stdin;

fn main() {
    println!("Конвертация из градусов по Фаренгейту в Цельсия");
    println!("Введите значения температуру по Фаренгейту, которую хотите конвертировать");
    loop {
        let mut user_input = String::new();
        stdin().read_line(&mut user_input).expect("Ошибка ввода");

        let far: f32 = match user_input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Вы ввели неверное значение, пожалуйста повторите попытку");
                continue;
            }
        };

        let cel :f32 = (far - 32f32) * (5f32 / 9f32);
        println!("{far}F = {cel}C");
    }
}
