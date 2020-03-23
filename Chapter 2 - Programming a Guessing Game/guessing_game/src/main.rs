/*  In order to obtain user input we must import the io library.        **
**  The IO library comes from the standary library (known as std)       */
use std::io;

/*  In order to generate random number we will be using the rand        **
**  library imported using Cargo.                                       */
use rand::Rng;

use std::cmp::Ordering;

fn main() {
    /*  The println!() macro prints a string to the console             */
    println!("Guess the number!");

    /*  Next we will create a place to store the user's input.          **
    **                                                                  **
    **  A let statement is used to create a variable                    **
    **                                                                  **
    **  In Rust, variables are immutable by default.                    **
    **  By adding mut before the variable you can make the              **
    **  variable mutable.                                               **
    **                                                                  **
    **      let a = 5;      //immutable                                 **
    **      let mut b = 12; //mutable                                   **
    **                                                                  */
    let mut guess;

    /*  Create an immutable variable "secret_number" to store the       **
    **  secret value.                                                   */
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("The secret number is: {}", secret_number);

    
    
    loop {
        /*  String is a string type provided by the standard library.       **  
        **  The :: syntax in the String::new() line indicates that new      **
        **  is an associated function of the String type.                   **
        **                                                                  **
        **  Associated functions are implemented on a specific type, in     **
        **  this case a String, rather than on a particular instance of a   **
        **  String. Some languages call this a static method.               */
        guess = String::new();
        
        println!("Please input your guess.");
        /*  Recall how earlier we included the use of the io library    **
        **  by importing it from the standard library. Now we'll call   **
        **  stdin from the io library using io::stdin()                 **
        **                                                              **
        **  If we had not included the line                             **
        **                                                              **
        **      use std::io                                             **
        **                                                              **
        **  then we could have used std::io::stdin()                    **
        **                                                              **
        **  The next part of the line is the .read_line method which    **
        **  gets called on the standard input handle to get input from  **
        **  the user.                                                   **
        **                                                              **
        **  Additionally we're passing the argument (&mut guess)        **
        **  The job or read_line is to take whatever the user types     **
        **  into standard input and places that into a string.          **
        **  The String argument must be mutable so the method can       **
        **  change the content of the String.                           **
        **                                                              **
        **  The & indicates that the argument is a reference.           **
        **  A reference is a way to allow multiple parts of your code   **
        **  access your data without needing to copy that data into     **
        **  memory multiple times.                                      **
        **  Like variables, references are immutable by default.        **
        **  That is why you need to do &mut guess, rather than &guess.  */
        io::stdin().read_line(&mut guess)

        /*      Handling Potential Failure with the Result Type         **
        **                                                              **
        **  As mentioned earlier, the read_line function puts what the  **
        **  user types into a String but it also returns something.     **
        **                                                              **
        **  Introducing the Result Type                                 **
        **  Result Types are enumerations (often called enums).         **
        **  Enums are a type with a fixed number of values              **
        **  called variants.                                            **
        **                                                              **
        **  For Result the variants are "Ok" and "Err"                  **
        **  The "Ok" variant indicates that the operation was           **
        **  successful and inside "Ok" is the successfully generated    **
        **  value.                                                      **
        **  The "Err" variant means the operation failed and            **
        **  inside "Err" contains information on how or why the         **
        **  operation failed.                                           */
            .expect("Failed to read line");

        /*  Here we create a variable called guess that uses a feature  **
        **  called shadowing.                                           **
        **  Shadowing allows you to convert a variable from one type to **
        **  another without needing to create a second variable         **
        **  
        **  We bind "guess" to the expression "guess.trim().parse()"    **
        **                                                              **
        **  The .trim method will remove any whitespace before and      **
        **  and after the string.                                       **
        **                                                              **
        **  The .parse method will convert the String into some kind of **
        **  number. Because this method can parse a variety of          **
        **  number types we need to tell Rust exactly what type of      **
        **  number we want by useing "let guess: u32"                   **
        ** */
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_)  => continue,
        };
        /*  This line prints the string we saved the user's input **
        **  in.                                                         **
        **                                                              **
        **  The set of {} is a placeholder.                             **
        **  You can print more than one value using curly brackets:     **
        **  the first set of curly brackets holds the first value       **
        **  listed after the format string, the second set holds the    **
        **  second value, and so on.                                    */
        println!("You guessed: {}", guess);

        /*  This match expression decides what to do next based on what **
        **  variant of Ordering it recieves.                            **
        **  
        **  A match expression is made up of arms. An arm consists of   **
        **  a pattern and the code that should be run if the value      **
        **  given to the beginning of the match expression fits that    **
        **  arm’s pattern. Rust takes the value given to match and      **
        **  looks through each arm’s pattern in turn.                   */
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }   
        }
    }
}
