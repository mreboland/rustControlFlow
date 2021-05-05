fn main() {
    // IF / ELSE EXPRESSIONS

    // The form of an if expression is a condition expression followed by a consequent block. Any number of else if conditions and blocks, and an optional trailing else block. The condition expressions must have a type bool.

    // The condition of if is the expression 1 == 2, which evaluates into a boolean type with the value false
    if 1 == 2 {
        println!("Whoops, mathematics broke");
    } else {
        println!("everything's fine!");
    }

    // Unlike in most languages, if blocks can also act as expressions. Remember that all branches must return the same type for our code to compile.

    let formal = true;
    // Checking if formal is true, which is is, so we print out Good evening.
    let greeting = if formal {
        "Good evening."
    } else {
        "Hello, friend!"
    };
    println!("{}", greeting); // prints "Good evening"

    // We can have multiple conditions by combining if and else in an else if expression.

    let number = 6;

    if number % 4 == 0 {
        println!("Number is divisible by 4");
    } else if number % 3 == 0 {
        println!("Number is divisible by 3");
    } else if number % 2 == 0 {
        println!("Number is divisible by 2");
    } else {
        println!("Number is not divisible by 4, 3, or 2");
    };
    // If a condition expression evaluates to true, the consequent block is executed. Any subsequent else if or else block is skipped. Because 6 is divisible by 3 it runs and everything else is skipped.

}
