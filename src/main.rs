use std::io;

fn main() {
    println!("Welcome!");
    
    let mon: (u32, str&, u32, u32, str&, str&, u32, str&, str&, bool) = (26, "Raichu", 1247, 1558, "Rainy", "", 3, "Electric", "", 1);

    println!({mon.0}, {mon.1}, {mon.2}, {mon.3}, {mon.4}, {mon.5}, {mon.6}, {mon.7}, {mon.8}, {mon.9});
}
