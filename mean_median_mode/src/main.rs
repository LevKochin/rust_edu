use std::collections::HashMap;
use rand::Rng;

fn main() {
    let mut colleciton: Vec<i32> = Vec::new();
    randomize_collection(&mut colleciton);
    colleciton.sort();
    print_collection(&colleciton);

    let mean = mean(&colleciton);
    println!("Mean: {}", mean);
    let median = median(&colleciton);
    println!("Median: {}", median);

    println!("Modes of collection is: ");
    let mode_collection: HashMap<i32, i32> = mode(&colleciton);
    for (key, value) in mode_collection {
        println!("Key: {}, Value: {}", key, value);
    }
}

fn print_collection(collection: &Vec<i32>) {
    print!("[ ");
    for item in collection {
        print!("{} ", item);
    }
    print!("]");
    println!();
}

fn randomize_collection(collection: &mut Vec<i32>) {
    let mut rng = rand::rng();
    for _ in 1..10 {
        let random_number = rng.random_range(1..10);
        collection.push(random_number);
    }
}

fn mean(collection: &Vec<i32>) -> i32 {
    let mut sum = 0;
    for item in collection {
        sum += item;
    }

    sum / collection.len() as i32
}

fn median(collection: &Vec<i32>) -> i32 {
    if collection.len() % 2 == 0 {
        collection[collection.len() / 2] + collection[collection.len() / 2 - 1] / 2
    } else {
        let index_of_median = collection.len() / 2;
        collection[index_of_median]
    }
}

fn mode(collection: &Vec<i32>) -> HashMap<i32, i32> {
    let mut collection_hash : HashMap<i32, i32> = HashMap::new();
    for &item in collection {
        let count = collection_hash
            .entry(item)
            .or_insert(0);
        *count += 1;
    }

    let max_count = collection_hash
        .values()
        .max()
        .cloned()
        .unwrap_or(0);

    let mut result : HashMap<i32, i32> = HashMap::new();
    for (&key, &value) in collection_hash.iter() {
        if value == max_count {
            result.insert(key, value);
        }
    }

    result
}
