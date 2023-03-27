use std::fmt::Display;

#[derive(Debug)]
struct User {
    name: &'static str,
    age: u64,
    /**
     * male | female
     **/
    gender: String,
}

#[derive(Debug)]
enum RGBenum {
    R(u8),
    G(u8),
    B(u8),
}

#[derive(Debug)]
 struct RGB (u8, u8, u8);


// impl Display for RGB {
    // fn fmt(&self, , &mut Formatter<'_>) {
        // println!("{} {} {}", self.0, self.1, self.2)
    // };
// }

impl User {
    fn call (name: &'static str, age: u64) -> User {
        User {
            name: name,
            age,
            gender: String::from("male"),
        }
    }
    fn myname (&self) -> &Self {
        println!("{} {} {}", self.age, self.name, self.gender);
        &self
    }
    fn callEnum (arg: RGBenum) {

    }
}


pub fn test() { 

    let usr = User::call("Teo", 15);
    let m = usr.myname();

    let color = RGB(255, 255, 255);

    let opt: Option<u32> = Some(0);
    match opt {
        Some(v) => println!("{}", v),
        None => println!("none", ),
    }

    let b: RGBenum = RGBenum::B(32);

    match b {
        RGBenum::B(v) => println!("blue {}", v),
        _ => println!("other"),
    }

    if let RGBenum::B(32) = b {
        println!("match let {:?}", b)
    }
    
    println!("{:?} {:?} {:?}", m, color, RGBenum::B(32));
}
