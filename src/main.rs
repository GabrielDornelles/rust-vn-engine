use visual_novel::{
    SceneContent, 
    load_json_scenes, render_dialogue_and_return_next, render_choices_and_return_next
};

fn main() {
    // functions that return Result<t,e> have to be unwrapped, expect does is with the custom error message
    let scenes_map = load_json_scenes().expect("failed to load scenes");
    let mut current_id = String::from("Intro");
    loop {
        
        let scene = scenes_map
        .get(&current_id)
        .expect("scene id referenced by next does not exist");

        match &scene.content { // because unwraping returns a borrow we cant access anything from the borrow, but we can borrow inner elements
            SceneContent::Dialogue { speaker, text, next } => {
                let next_scene = render_dialogue_and_return_next(&speaker, &text, next.clone());
                
                match next_scene {
                    Some(next_id) => current_id = next_id,
                    None => {
                        println!("End of Story.");
                        break;
                    }
                }
            }
            SceneContent::Choice {speaker, text, options} => {
                let next_scene = render_choices_and_return_next(&speaker, &text, options);
                
                match next_scene {
                    Some(next_id) => current_id = next_id,
                    None => {
                        println!("End of Story.");
                        break;
                    }
                }

            }
           
        };
    }
}