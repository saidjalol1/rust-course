
fn main(){

    /*
    Operators : 
                OR  ==> | 
                AND ==> &
                XOR ==> ^
                COMPARISON OPERATORS :
                            ==  => equal to
                            <  => less than
                            >  => greater than
                            <=  => less than or equal to
                            >=  => greater than or equal to
     */

    let greater_than: bool = 10 < 5;
    let not_equals: bool = 1 != 0;
    let equals: bool = "0x5ff" == "0x5ff";
    // true and false are actually  just 1 and 0 respectivly;
    let combined: bool = greater_than && not_equals || equals > true;
    println!("{}", combined); // prints 'false'


    let n: u8 = 1;

    if n % 2 == 0 {
        println!("n = {} , which is even number!", n);
    } else if n == 1 {
        println!("n = {} , which is special case!",n);
    }else{
        println!("n = {} , which is odd number!",n);
    }

    // expressions ===> using if blocks as expressions requireds both if and else block and should return values respectively ,
    // to return valye you need to remove ';' from the last expression in the value 
    let expression_result: String = if n == 1 {"one".to_string()} else {"not one".to_string()};
    println!("Result : {}", expression_result);

    

}