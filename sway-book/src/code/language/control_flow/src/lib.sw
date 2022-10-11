library control_flow;

fn conditional() {
    // ANCHOR: conditional
    let number = 5;

    if number % 3 == 0 {
        // call function 1
    } else if number % 4 == 0 {
        // call function 2
    } else {
        // call function 3
    }

    // more code here
    // ANCHOR_END: conditional
}

// ANCHOR: compute
fn compute(deposit: u64) {
    let minimum_deposit_met = if 10 < deposit { true } else { false };
    // code
}
// ANCHOR_END: compute

fn single_loop() {
    // ANCHOR: single_loop
    let mut counter = 0;
    while counter < 10 {
        counter += 1;
    }
    // ANCHOR_END: single_loop
}

fn nested_loop() {
    // ANCHOR: nested_loop
    while true {
        // computation here
        while true {
            // more computation here
        }
    }
    // ANCHOR_END: nested_loop
}

// ANCHOR: break_example
fn break_example() -> u64 {
    let mut counter = 1;
    let mut sum = 0;
    let num = 10;
    while true {
        if counter > num {
            break;
        }
        sum += counter;
        counter += 1;
    }
    sum // 1 + 2 + .. + 10 = 55
}
// ANCHOR_END: break_example

// ANCHOR: continue_example
fn continue_example() -> u64 {
    let mut counter = 0;
    let mut sum = 0;
    let num = 10;
    while counter < num {
        counter += 1;
        if counter % 2 == 0 {
            continue;
        }
        sum += counter;
    }
    sum // 1 + 3 + .. + 9 = 25
}
// ANCHOR_END: continue_example

// ANCHOR: if_let_enum
enum Foo {
    One: (),
    Two: (),
}
// ANCHOR_END: if_let_enum

fn example1() {
    // ANCHOR: if_let_example1
    let one = Foo::One;
    let mut result = 0;
    
    if let Foo::One = one {
        result = 1;
    }
    // ANCHOR_END: if_let_example1
}

fn example2() {
    // ANCHOR: if_let_example2
    let one = Foo::One;
    let result = if let Foo::One = one {
        1
    } else {
        2
    };
    // ANCHOR_END: if_let_example2
}


fn simple_match() {
    // ANCHOR: simple_match
    let number = 5;

    let result = match number {
        0 => 10,
        1 => 20,
        5 => 50,
        catch_all => 0,
    };
    // ANCHOR_END: simple_match
}

fn multi_line_match() {
    // ANCHOR: multi_line_match
    let number = 5;

    let result = match number {
        0 => {
            // Multiple lines of code here then return 10
            10
        },
        1 => 20,
        5 => 50,
        catch_all => 0,
    };
    // ANCHOR_END: multi_line_match
}

// ANCHOR: complex_enum_match
enum TopLevel {
    One: (),
    Two: SecondLevel,
}

enum SecondLevel {
    Value1: u64,
    Value2: (),
}

fn nested_match(input: TopLevel) -> u64 {
    match input {
        TopLevel::One => 1,
        TopLevel::Two(second) => {
            match second {
                SecondLevel::Value1(2) => 2,
                SecondLevel::Value1(_) => 3,
                SecondLevel::Value2 => 42,
            }
        },
    }
}
// ANCHOR_END: complex_enum_match
