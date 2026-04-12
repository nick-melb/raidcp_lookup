use serde::Deserialize;
use std::error::Error;
use std::io::{self, Write};

#[derive(Debug, Deserialize)]
struct Pokemon {
    number: u32,         // Handles IDs like 1, 01, 001 identically
    name: String,
    form: String,        // e.g., "Normal", "Alolan", "Mega Y"
    hundo_cp: u32,
    hundo_wb_cp: u32,
    weather0: String,
    weather1: Option<String>,
    raid_tier: String,
    type0: String,
    type1: Option<String>,
    shiny_enable: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    // Bake the CSV into the binary. 
    // Ensure "pokemon.csv" is in your project root.
    let csv_data = include_str!("data.csv");

    println!("Pokemon Go Raid Hundo CP Lookup Tool");
    println!("Enter Name or Number of the Pokemon | 'exit' or CTRL-cto quit");

    loop {
        print!("\nSearch: ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let query_raw = input.trim();
        if query_raw.is_empty() {
            continue;
        }
        let query_lowercase = query_raw.to_lowercase();

        if query_lowercase == "exit" {
            break;
        }

        // Parse number if possible (e.g., "038" -> Some(38))
        let query_as_num: Option<u32> = query_raw.parse().ok();

        let mut rdr = csv::Reader::from_reader(csv_data.as_bytes());
        let mut found = false;

        for result in rdr.deserialize() {
            let p: Pokemon = result?;

            let is_name_match = p.name.to_lowercase().contains(&query_lowercase);
            let is_num_match = query_as_num.map_or(false, |n| n == p.number);

            if is_name_match || is_num_match {
                // Logic to put Form in front of Name, unless it's "Normal"
                let display_name = if p.form.to_lowercase() == "normal" {
                    p.name.clone()
                } else {
                    format!("{} {}", p.form, p.name)
                };

                println!("\n[#{:03} {}]", p.number, display_name);
                println!("Raid Tier:                     {}", p.raid_tier);
                println!("Hundo:                         {}CP", p.hundo_cp);
                let w1 = p.weather1.as_deref().unwrap_or("None");
                println!("If the Weather is:             {} / {}", p.weather0, w1);
                println!("Weather Boosted Hundo will be: {}CP", p.hundo_wb_cp);
                let t1 = p.type1.as_deref().unwrap_or("None");
                println!("Types:                         {} / {}", p.type0, t1);
                
                println!("Shiny:                         {}", if p.shiny_enable { "Enabled" } else { "No" });
                
                found = true;
                // We don't 'break' so we can find multiple forms (Normal & Alolan)
            }
        }

        if !found {
            println!("No results found for '{}'", query_raw);
        }
    }

    Ok(())
}

