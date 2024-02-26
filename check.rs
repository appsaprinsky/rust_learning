fn main() {
    'qouter: loop {
        println!("Entered the outer loop");

        'qinner: loop {
            println!("Entered the inner loop");

            // This would break only the inner loop
            //break;

            // This breaks the outer loop
            break 'qouter;
        }

        println!("This point will never be reached");
    }

    println!("Exited the outer loop");
}