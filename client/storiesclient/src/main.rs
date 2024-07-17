use anyhow::anyhow;
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs, io, str::FromStr};

// create a new struct (custom data type)
// we also add the Serialize and Deserialize traits in order to convert json data into a Story
// struct.
// Using derive allows us to easily implement functionality for our own struct (or enum)
#[derive(Serialize, Deserialize)]
struct Story {
    adventure: Vec<String>,
    romcom: Vec<String>,
    family: Vec<String>,
    fantasy: Vec<String>,
}

// create a function called get_genre for that struct
impl Story {
    fn get_genre(&self, genre: Genre) -> &[String] {
        match genre {
            Genre::Adventure => &self.adventure,
            Genre::RomCom => &self.romcom,
            Genre::Family => &self.family,
            Genre::Fantasy => &self.fantasy,
        }
    }
}

// enum in rust,
// we use the derive attribute to be able to use the Copy and Clone traits
#[derive(Copy, Clone)]
enum Genre {
    Adventure,
    RomCom,
    Family,
    Fantasy,
}

impl FromStr for Genre {
    type Err = anyhow::Error;

    // parse function for the Genre enum
    // returns a result object (Genre if successful, otherwise a String)
    // this is how Rust handles errors and makes you handle any errors that you could encounter when
    // this is called
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().trim() {
            "ADVENTURE" => Ok(Genre::Adventure),
            "ROMCOM" => Ok(Genre::RomCom),
            "FAMILY" => Ok(Genre::Family),
            "FANTASY" => Ok(Genre::Fantasy),
            _ => Err(anyhow!(
                "Please enter a valid story genre: adventure, romcom, family, or fantasy",
            )),
        }
    }
}

fn main() -> anyhow::Result<()> {
    println!("Select a type of story (adventure, romcom, family, fantasy): ");

    // some great examples of error handling in rust and the match (switch) operation
    let story_genre = stdin_line()?.parse()?;
    let contents = fs::read_to_string("./data/stories.json")?;
    let data: Story = serde_json::from_str(&contents)?;

    // next we need to get all the <> fields and then ask the user to replace them
    let mut chosen_story = data
        .get_genre(story_genre)
        .choose(&mut rand::thread_rng())
        .expect("all story types should have at least one story")
        .clone();
    println!("Random pick: {}", chosen_story);

    let mut replacements = HashMap::new();
    for placeholder in regex::Regex::new("<[^>]+>")?
        .find_iter(&chosen_story)
        .map(|m| m.as_str())
    {
        println!("Enter a {}: ", placeholder);
        let user_input = stdin_line()?;
        replacements.insert(
            placeholder.to_string(),
            user_input.clone().trim().to_string(),
        );
        println!("{placeholder}: {user_input}");
    }

    for (placeholder, replacement) in replacements.iter() {
        println!("old word: {}", placeholder);
        println!("new word: {}", replacement);
        // need to replace each of the place holder values with the new word.
        chosen_story = chosen_story.replace(placeholder, replacement);
    }

    println!("Your new story:\n{}", chosen_story);

    Ok(())
}

fn stdin_line() -> io::Result<String> {
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input)?;
    Ok(user_input)
}
