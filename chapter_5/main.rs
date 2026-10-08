fn main() {
    let owner_a = String::from(“ ”);
    let owner_b = owner_a;
    {
        let owner_c = String::from(“ ”);
    } // owner_c will be dropped here
    let owner_d = owner_c; // this won’t work


    // This works 
    let owner_e = {
        let owner_f = String::from(“ ”);

        owner_f
    };

    
}
