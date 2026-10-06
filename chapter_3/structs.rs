


struct Door{
    width: u32,
    height: u32,
    is_open: bool
}

// making the struct and struct's certain attributes accessable from outside of the module
pub struct PublicDoor{ // this pub keywors makes the struct accessable in other modules
    width: u32, // when using this struct in other module this
    height: u32, // and this are not accessable 
    pub is_open: bool // only this one is 
}

impl Door {
    pub fn new(width: u32,height:u32, is_open:bool) -> Self {
        Door {width,height,is_open}
    }

    pub fn open(&mut self) {
        self.is_open = true;
    }

    pub fn close(&mut self) {
        self.is_open = false;
    }
}
// snXjUKrsIdt0g
// P4-To6=Zen4
fn main(){

    // storing data about door in sepearate vars
    let _door_width = 100;
    let _door_height = 200;
    let _door_is_open = false;

    
    // storing data about the door using 'struct' type
    let _living_room_door = Door{width: 100, height:200, is_open:true};
    
    // initializing same struct with its impl
    let mut new_door = Door::new(100, 200, false);

    // calling the impl method 
    new_door.open();
    assert!(new_door.is_open); // crashed when false
}

