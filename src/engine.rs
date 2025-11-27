use crate::scene::{ChoiceOption, Scene};
use crate::parser::get_next_from_user_choice;
use std::collections::HashMap;
use std::error::Error;

pub fn load_json_scenes() -> Result<HashMap<String, Scene>, Box<dyn Error>> {
    let data = std::fs::read_to_string("story.json")?;
    let list: Vec<Scene> = serde_json::from_str(&data)?;
    // Map by scene_id field for easy lookup:
    Ok(list.into_iter().map(|s| (s.scene_id.clone(), s)).collect())
}

pub fn render_dialogue_and_return_next(
    speaker: &str, 
    text: &str, 
    next: Option<String>
) -> Option<String> {
    println!("{speaker}: {text}");
    next
}

pub fn render_choices_and_return_next(
    speaker: &str, 
    text: &str, 
    options: &[ChoiceOption] //slices are easier to use than vec
) -> Option<String> {
    println!("{speaker}: {text}");
    for (i, item) in options.iter().enumerate() {
        println!("{i}: {0}", item.description);
    }
    get_next_from_user_choice(options)
}