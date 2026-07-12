use format_me::*;

fn main() {
    println!(
        "{}",
        Park {
            name: Some("Les Tuileries".to_string()),
            park_type: ParkType::Garden,
            address: Some("Pl. de la Concorde".to_string()),
            cap: Some("75001".to_string()),
            state: Some("France".to_string())
        }
    );
    println!(
        "{}",
        Park {
            name:Some( "".to_string()),
            park_type: ParkType::Playground,
            address: Some("".to_string()),
            cap: Some("".to_string()),
            state: Some("".to_string()),
        }
    );
}
