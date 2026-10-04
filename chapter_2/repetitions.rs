

fn main(){


    let mut i  = 0;

    
    // This is infinite loop which never stops , it repeats the block you wrote
    // loop {
    //     i += 1;
    //     println!("{} value is incremented", i);
    // }

    // Loop with conditions and break and continue operators
    loop {
        i += 1;
        if i == 100{
            break;
        }else if i % 2 == 0{
            continue;
        }else{
            println!("{} odd number!", i);
        }
    }

    // inner loops and labeling them 

    let mut loop_var: u8 = 0;
    'main: loop {
        'inner: loop {
            if loop_var < 50{
                loop_var += 1;
                println!("{} inner loop value!", loop_var);
            }else{
                println!("Exiting Inner loop ....");
                break 'inner;
            };
        }

        if loop_var < 200 && loop_var >= 50 {
            loop_var += 1;
            println!("{} outer loop value!", loop_var);
        }else{
            println!("Exiting Outer loop ....");
            break 'main;
        }
    }


    // For loops , iterating collections with for loops
    for num in 9..100{  // Excluding 100
        println!("num is {}", num);
    }

    for num in 9..=20{ // including 20
        println!("num is {}", num);
    }

    for num in &[7,5,2,45,67,43,23,89,65,12]{ // iterating collections
        if num % 2 == 0{
            println!("Valid num {}", num);
        }
    }

    // bubble sort with for loop
    let mut array = [7,5,2,45,67,43,23,89,65,12];
    for _ in 0..array.len(){
        let mut _swaps = 0;

        for i in 1..array.len(){
            if array[i - 1] > array[i]{
                let tmp = array[i];
                array[i] = array[i - 1];
                array[i - 1] = tmp;
                _swaps += 1;
            }
        }
    }
    println!("Sorted array with bubble sort : {:?}", array);



    // while 
    let mut while_var = 0;
    while while_var <= 10 {
        while_var += 1;
        println!("{} value ", while_var);
    }
}