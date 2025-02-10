use std::collections::HashMap;
use std::io::stdin;
fn main() {
    println!("Интерфейс управления сотрудниками предприятия");
    println!();
    println!("-h или --help для вызова доступных команд");
    let mut subunit_employers : HashMap<String, String> = HashMap::new();
    let mut user_input = String::new();
    loop {
        user_input.clear();
        match stdin().read_line(&mut user_input){
            Ok(input) => input,
            Err(_) => {
                println!("Вы допустили ошибку, посмотрите список доступных команд с помощью -h или --help");
                continue;
            }
        };

        let user_input = user_input.trim();
        if user_input == "-h" || user_input == "--help" {
            println!("Добавление нового сотрудника в подразделение");
            println!("add <name surname patronymic> to <subunit>");
            println!();
            println!("Вывод списка всех сотрудников по подразделениям сортированных по алфавиту");
            println!("get employee");
            continue;
        }

        if user_input.contains("add") && user_input.contains("to") {
            let mut is_employee = false;
            let mut temp_employee = String::new();
            let mut temp_subunit = String::new();
            for command in user_input.trim().split_whitespace() {
                if command == "add" {
                    is_employee = true;
                    continue;
                }

                if command == "to" {
                    is_employee = false;
                    continue;
                }

                if is_employee {
                    temp_employee.push_str(command);
                } else {
                    temp_subunit.push_str(command);
                }
            }

            subunit_employers
                .entry(temp_employee.clone())
                .or_insert(temp_subunit.clone());
            println!("{} успешно добавлена к {}", temp_employee, temp_subunit);
        } else if user_input == "get employee" {
            println!("Список сотрудников и их принадлежность к подразделению");
            let mut employee_collection: Vec<(&String, &String)> =  subunit_employers.iter().collect();
            employee_collection.sort_by(|a, b| a.1.cmp(b.1));
            for (employee_name, subunit) in &employee_collection
            {
                println!("{}: {} ", employee_name, subunit);
            }
        };
    }

}
