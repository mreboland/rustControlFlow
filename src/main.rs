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

    // LOOPING WITH LOOP

    // A loop expression denotes an infinite loop. It repeats execution of its body continuously.

    // loop {
    //     println!("I loop forever!");
    // };

    // Unlike other kinds of loops in Rust like while and for, loop can be used in expressions that return values via break.

    let mut i = 1;

    let something = loop {
        i *= 2;
        if i > 100 {
            break i;
        }
    };
    assert_eq!(something, 128);

    // Every break in a loop must have the same type. When it's not explicitly giving something, break; return (), and empty tuple.

    // LOOP UNTIL A CRITERIA IS MET WITH WHILE LOOPS

    // A while expression loops until a predicate is false.
    // If the loop conditional expression evaluates to true, the loop body block executes. Control then returns to the loop conditional expression. If the loop conditional expression evaluates to false, the while expression completes.

    let mut counter = 0;

    while counter < 10 {
        println!("Hello, {}", counter);
        counter = counter + 1;
    };

    // ITERATE WITH FOR LOOPS

    // A for expression extracts values from an iterator. It loops until the iterator is empty.
    // In Rust, an iterator is any type that can iterate over values. Some values can be iterated over directly and others can produce iterators by calling methods like .iter().

    let a = [10, 20, 30, 40, 50];

    // The code iterates through each element in the array and binds it to the element variable. The println! macro then prints each of those values in sequence.
    for element in a.iter() {
        println!("The value is: {}" , element);
    };

    // Another easy way to create an iterator is to use the range notation a..b. This notation yields values from a (inclusive) to b (exclusive) in steps of one.

    for item in 0..5 {
        println!("{}", item *2);
    };

}

