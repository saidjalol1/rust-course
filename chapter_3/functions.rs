


/*
Types of parameters:
            1.  a: &str  ===> immutable referece (a borrow)
            2.  mut a: String ===> mut makes the input var mutable
            3.  a: u32   ===> this is transferring the ownership to the functions
            4.  a: impl  ===> which is handy for allowing structs/enums that implement a trait dynamically
            5.  a: T ===> other generic types are represented through letters  

*/

fn four() -> u32{  // function without parameter, which always returns 4
    4
}

