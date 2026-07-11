// pub enum AccessLevel {
//     Guest,
//     Normal,
//     Admin,
// }

// pub struct User {
//     name: String,
//     access_level: AccessLevel,
// }

// impl User {
//     pub fn new(name: String, level: AccessLevel) -> User {
//         User {
//             name,
//             access_level: level,
//         }
//     }

//     pub fn send_name(&self) -> Option<&str> {
//         match self.access_level {
//             AccessLevel::Guest => None,
//             _ => Some(&self.name),
//         }
//     }
// }

// pub fn check_user_name(user: &User) -> (bool, &str) {
//     match user.send_name() {
//         Some(name) => (true, name),
//         None => (false, "ERROR: User is guest"),
//     }
// }  




#[derive(PartialEq)]
pub enum AccessLevel {
Guest,
Normal,
Admin,

}

pub struct User {
    name : String,
    acessLevel : AccessLevel,

}

impl User {
  pub fn new(name: String, level: AccessLevel) -> User {
    User{
        name : name,
        acessLevel : level,

    }

  }
  pub fn send_name(&self) -> Option<&str> {

        if self.acessLevel ==AccessLevel::Guest {

        return     None
        }
        return Some(&self.name)


  }
}

pub fn check_user_name(user: &User) -> (bool, &str) {
    if user.send_name() != None {
        (true, user.send_name().expect("REASON"))
    } else {
        (false, "ERROR: User is guest")
    }
}