# rust_edu

## Образовательная сборка проектов для освоения Rust

### Базовые упражнения взяты из https://doc.rust-lang.ru/book/ch03-05-control-flow.html

1. [Celsius To Fahrenheit Converter](./c_to_f_converter)
2. [Fibonacci](./fibonacci)
3. [Twelve Days of Christmas](./cristmas_song)

Перед выполнением, изучены темы:

1. [Установка и запуск](https://doc.rust-lang.ru/book/ch01-00-getting-started.html)
2. [Программирование игры в загадку](https://doc.rust-lang.ru/book/ch02-00-guessing-game-tutorial.html). Ссылка на
   образовательный ресурс.
3. [Общие концепции программирования](https://doc.rust-lang.ru/book/ch03-00-common-programming-concepts.html)

### В процессе выполнения

Упражнения на главе 8.3 Общие коллекции https://doc.rust-lang.ru/book/ch08-00-common-collections.html
Перед выполнением необходимо изучить от и до темы в диапазоне от 4 до 8.3
Начало темы -> https://doc.rust-lang.ru/book/ch04-00-understanding-ownership.html

#### Задача 1

Описание задачи

```text
Есть список целых чисел. Создайте функцию, используйте вектор и верните из списка: среднее значение; 
медиану (значение элемента из середины списка после его сортировки); 
моду списка (mode of list, то значение которое встречается в списке наибольшее количество раз;
HashMap будет полезна в данном случае).
```

[Решение](https://github.com/LevKochin/rust_edu/tree/processing/mean_median_mode)

#### Задача 2

Описание задачи

```text
Преобразуйте строку в кодировку "поросячьей латыни" (Pig Latin).
Первая согласная каждого слова перемещается в конец и к ней добавляется окончание "ay", так "first" станет "irst-fay". 
Слову, начинающемуся на гласную, в конец добавляется "hay" ("apple" становится "apple-hay"). 
Помните о деталях работы с кодировкой UTF-8!
```

#### Задача 3

Описание задачи

```text
Используя хеш-карту и векторы, создайте текстовый интерфейс позволяющий пользователю добавлять 
имена сотрудников к названию отдела компании. Например, "Add Sally to Engineering" или "Add Amir to Sales".
Затем позвольте пользователю получить список всех людей из отдела или всех людей в компании, 
отсортированных по отделам в алфавитном порядке.
```
