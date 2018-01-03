fn main() {
    
    for count in 0..100 {

        if count%3 == 0 && count%5 == 0 {
            println!("fizzbuzz"); 
        } else if count%3 == 0 {
            println!("fizz");
        } else if count%5 == 0 {
            println!("buzz"); 
        } else {
            println!("{}", count);
        }
    
    }

}
