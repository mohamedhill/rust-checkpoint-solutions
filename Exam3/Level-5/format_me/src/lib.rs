use std::fmt;

pub struct Park {
    pub name: Option<String>,
    pub park_type: ParkType,
    pub address: Option<String>,
    pub cap: Option<String>,
    pub state: Option<String>,
}

pub enum ParkType {
    Garden,
    Forest,
    Playground,
}

impl fmt::Display for Park {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} - {}, {}, {} - {}",
            self.park_type,
            self.name.as_deref().unwrap_or("No name"),
            self.address.as_deref().unwrap_or("No address"),
            self.cap.as_deref().unwrap_or("No cap"),
            self.state.as_deref().unwrap_or("No state"),
        )
    }
}

impl fmt::Display for ParkType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParkType::Garden => write!(f, "garden"),
            ParkType::Forest => write!(f, "forest"),
            ParkType::Playground => write!(f, "playground"),
        }
    }
}

#[test]
fn test_park() {
    let park = Park {
        name: Some("Central Park".to_owned()),
        park_type: ParkType::Garden,
        address: Some("Av. Sidónio Pais 4".to_owned()),
        cap: Some("1050-214".to_owned()),
        state: Some("Portugal".to_owned()),
    };

    assert_eq!(
        park.to_string(),
        "garden - Central Park, Av. Sidónio Pais 4, 1050-214 - Portugal"
    );
}

#[test]
fn test_empty_name() {
    let park = Park {
        name: None,
        park_type: ParkType::Forest,
        address: Some("Av. Sidónio Pais 4".to_owned()),
        cap: Some("1050-214".to_owned()),
        state: Some("Portugal".to_owned()),
    };

    assert_eq!(
        park.to_string(),
        "forest - No name, Av. Sidónio Pais 4, 1050-214 - Portugal"
    );
}

#[test]
fn test_empty_all() {
    let park = Park {
        name: None,
        park_type: ParkType::Playground,
        address: None,
        cap: None,
        state: None,
    };

    assert_eq!(
        park.to_string(),
        "playground - No name, No address, No cap - No state"
    );
}