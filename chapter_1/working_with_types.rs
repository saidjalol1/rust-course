fn main(){


    // String type
    let text:String = "Hello , world".to_string();
    println!("{}", text);


    /*
            Integers - In rust there are many two types of integers signed and unsigned and they 
            and both have many other types as well 

            1) signed itegers can store both negative and positive numbers 
            2) unsigned integers can only store postive numbers.

    */

    // 1. UNSIGNED INTEGERS =======>  u8, u16, u32 , u64, u128
    let _age : u8 = 1;
    let _price : u16 = 10;
    let _amount: u32 = 100;
    

    // 2. SIGNED INTEGERS =======>  i8, i16, i32, i64, i128 
    let _age : i8 = -1;
    let _price : i16 = 0;
    let _amount: i32 = -99999;
    let _number: i32 = 9999;


    // Char type - stores character 
    let _emoji: char = '😎';
    let _letter : char = 'b';

    // bool type - stores true , false
    let _active : bool = true;

    // () ====> Tuple , if its empty which means it is Unit type
    let unit_type : () ;
    let numbers: (i32, u64) = (10, 45);


    // Arrays
    let array_elements: [u8, 4] = [1,2,3,4];



    
}   