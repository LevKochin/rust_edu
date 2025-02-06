fn main() {
    println!("Twelve Days of Christmas");
    println!();
    let chorus = ["On the", "day of Christmas,", "my true love sent to me", "And a partridge in a pear tree."];
    let days = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"];
    let song = ["Two turtle doves,", "Three French hens,", "Four calling birds,", "Five golden rings,",
    "Six geese a-laying,", "Seven swans a-swimming,", "Eight maids a-milking,", "Nine ladies dancing,", "Ten lords a-leaping,",
    "Eleven pipers piping", "Twelve drummers drumming"];
    for i in 0..days.len() {
        println!("{0} {1} {2} {3}", chorus[0], days[i], chorus[1], chorus[2]);
        for j in (0..i).rev() {
            println!("{}", song[j]);
        }

        println!("{}", chorus[3]);
        println!();
    }
}