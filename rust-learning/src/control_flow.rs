pub(crate) fn if_expressions(){
    let number=3;
    if number<5 {
        println!("condition true");
    }else {
        println!("condition false");
    }
}

pub(crate) fn if_else_expression(number:i32){
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}
pub(crate) fn one_line_if_else(){
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
}

pub(crate) fn basic_loop(){
    loop {
        println!("again!");
    }
}

pub(crate)fn returning_values_from_loop(){
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {result}");
}