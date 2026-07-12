// #[derive(Debug, PartialEq, Eq)]
// pub struct OfficeWorker {
//     name: String,
//     age: u32,
//     role: WorkerRole,
// }

// #[derive(Debug, PartialEq, Eq)]
// pub enum WorkerRole {
//     Admin,
//     User,
//     Guest,
// }

// impl From<&str> for WorkerRole {
//     fn from(role: &str) -> Self {
//         match role.to_lowercase().as_str() {
//             "admin" => WorkerRole::Admin,
//             "user" => WorkerRole::User,
//             "guest" => WorkerRole::Guest,
//             _ => panic!("Invalid role"),
//         }
//     }
// }

// impl From<&str> for OfficeWorker {
//     fn from(s: &str) -> Self {
//         let parts: Vec<&str> = s.split(',').collect();
//         if parts.len() != 3 {
//             panic!("Invalid input format");
//         }

//         let name = parts[0].to_string();
//         let age = parts[1].parse::<u32>().expect("Invalid age");
//         let role = WorkerRole::from(parts[2]);

//         OfficeWorker { name, age, role }
//     }
// }






#[derive(Debug, PartialEq, Eq)]
pub struct OfficeWorker {

name :String,
age : i32 ,
role : WorkerRole,
}

#[derive(Debug, PartialEq, Eq)]
pub enum WorkerRole {
    Admin,
    User,
    Guest,
}

impl From<&str> for OfficeWorker {
   fn from(s : &str)->Self{
   let  parts:Vec<_> = s.split(',').collect();

    if parts.len()!= 3{
        panic!("Invalid role")
    }

    let name = parts[0].to_string();
    let age = parts[1].parse().unwrap();
    let role = WorkerRole::from(parts[2]);

    OfficeWorker{
        name : name,
        age : age ,
        role : role

    
    }





    }
    
}

impl From<&str> for WorkerRole {
    fn from(s : &str)->Self{

  let role =   match s {
    "admin"=>WorkerRole::Admin,
    "guest"=>WorkerRole::Guest,
    "user"=>WorkerRole::User,
    _ => panic!("Invalid role"),
    };
role


    }


}




#[test]
fn test_office_worker() {
    assert_eq!(
        OfficeWorker::from("Louise,25,admin"),
        OfficeWorker {
            name: "Louise".to_owned(),
            age: 25,
            role: WorkerRole::Admin,
        }
    );
    assert_eq!(
        OfficeWorker::from("Rob,11,guest"),
        OfficeWorker {
            name: "Rob".to_owned(),
            age: 11,
            role: WorkerRole::Guest,
        }
    );
    assert_eq!(
        OfficeWorker::from("Maria Agata,44,user"),
        OfficeWorker {
            name: "Maria Agata".to_owned(),
            age: 44,
            role: WorkerRole::User,
        }
    );
}

#[test]
fn test_worker_role() {
    assert_eq!(WorkerRole::from("guest"), WorkerRole::Guest);
    assert_eq!(WorkerRole::from("admin"), WorkerRole::Admin);
    assert_eq!(WorkerRole::from("user"), WorkerRole::User);
}