 #[derive(Debug, PartialEq, Eq)]
 pub struct Outfit {
    pub jacket: Jacket,
     pub hat: Hat,
 }
 #[derive(Eq,PartialEq,Debug)]
 pub enum Jacket{
Black,
 White,
 Flowers,

  }
 #[derive(Eq,PartialEq,Debug)]
pub enum Hat{
 Snapback,
Baseball,
Fedora


  }

pub fn choose_outfit(formality_level: Option<u32>, invitation_message: Result<&str, &str>) -> Outfit {
    let jacket = match formality_level {
        Some(level) if level > 0 => Jacket::White,
        Some(_) => Jacket::Black,
        None => Jacket::Flowers,
    };

    let hat = match invitation_message {
        Ok(_) => Hat::Fedora,
        Err(_) => Hat::Snapback,
    };

    if formality_level.is_none() && invitation_message.is_err() {
        return Outfit {
            jacket: Jacket::Flowers,
            hat: Hat::Baseball,
        };
    }

    Outfit { jacket, hat }
}