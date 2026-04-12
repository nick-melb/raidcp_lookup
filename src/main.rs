use serde::Deserialize;
//use std::error::Error;
//use std::fs::File;

struct Pokemon {
    number: &'static str,
    name: &'static str,
    hundo_cp: &'static str,
    hundo_wb_cp: &'static str,
    weather0: &'static str,
    weather1: &'static str,
    raid_tier: &'static str,
    type0: &'static str,
    type1: &'static str,
    shiny_enable: bool,
}

fn main() {
    //pull in csv at compile time
    let data = include_str!("data.csv");

    //csv reader
    let mut rdr = csv::Reader::from_reader(data.as_bytes());

    // sort data from data.csv
    for result in rdr.deserialize::<Pokemon>() {
        match result {
        Ok(p) => println! ("Found {} (Shiny: {})", p.name, p.shiny_enable),
        Err(e) => eprintln!("Error: {}", e),
        }
    }
}
// old data below
//fn main() {
//    let mon_list = vec!
//        Pokemon {
//    println!("Welcome!");
    
//    let mon: (u32, str&, u32, u32, str&, str&, u32, str&, str&, bool) = (26, "Raichu", 1247, 1558, "Rainy", "", 3, "Electric", "", 1);

  //  println!({mon.0}, {mon.1}, {mon.2}, {mon.3}, {mon.4}, {mon.5}, {mon.6}, {mon.7}, {mon.8}, {mon.9});
//}
