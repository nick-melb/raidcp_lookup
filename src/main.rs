use serde::Deserialize;
use std::error::Error;
use std::io::{self, Write};
//use std::io::{self};

#[derive(Debug, Deserialize)]
struct Pokemon {
    number: u32, // to ensure ID's are treated as numbers
    name: String,
    form: String, // for subtypes of same pokemon number, will printed before name
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
    //  import data.csv and compile in, maybe compress later when database increases
    let csv_data = include_str!("data.csv");

    println!("Pokemon Go Raid Hundo CP Lookup Tool\nEnter Pokemon's Name or Dex Number (partial anme matches are accepted)\n type 'exit' or ':q' or hold CTRL-C at anytime to quit");

    loop {
        print!("\nSearch: ");
        io::stdout().flush()?; //corrects user input location

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let query_raw = input.trim();
        if query_raw.len() > 32 {
            println!("Error: Input is too long. Search must be under 32 characters.");
            continue;
        }
        if query_raw.is_empty() {
            continue;
        }
        let query_lowercase = query_raw.to_lowercase();

        if query_lowercase == "exit" || query_lowercase == "quit" || query_lowercase == ":q" {
            break;
        }

        // Parse number ie 026 and 26
        let query_as_num: Option<u32> = query_raw.parse().ok();

        let mut rdr = csv::Reader::from_reader(csv_data.as_bytes());
        let mut found = false;

        for result in rdr.deserialize() {
            let p: Pokemon = result?;

            let is_name_match = p.name.to_lowercase().contains(&query_lowercase);
            let is_num_match = query_as_num.map_or(false, |n| n == p.number);

            if is_name_match || is_num_match {
                // form in front of name when not "Normal" in csv
                let display_name = if p.form.to_lowercase() == "normal" {
                    p.name.clone()
                } else {
                    format!("{} {}", p.form, p.name)
                };

                println!("\n[#{:03} {}]", p.number, display_name);
                println!("Raid Tier:                     {}", p.raid_tier);
                println!("Hundo:                         {}CP", p.hundo_cp);
                let w1 = p.weather1.as_deref().unwrap_or(" "); //annoying to work out
                println!("If the Weather is:             {}  {}", p.weather0, w1);
                println!("Weather Boosted Hundo will be: {}CP", p.hundo_wb_cp);
                let t1 = p.type1.as_deref().unwrap_or(" ");
                println!("Type(s):                       {}  {}", p.type0, t1);

                println!(
                    "Shiny:                         {}",
                    if p.shiny_enable { "Enabled" } else { "No" }
                );

                found = true;
                // will continue to find other forms
            }
        }

        if !found {
            println!(
                "No results found for '{}',\nplease try again or type exit to quit",
                query_raw
            );
        }
    }

    Ok(())
}
