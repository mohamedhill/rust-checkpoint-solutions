use own_and_return::*;



fn main() {
    let my_film = Film {
        name: "Terminator".to_string(),
    };

   // println!("{}", take_film_name(my_film));

    // This line will not compile because my_film was moved above.
    println!("{}", read_film_name(&my_film));

    println!("{}", take_film_name(my_film));
}