pub mod scene; // tells Rust to load src/scene.rs as a module called scene.
pub mod parser;
pub mod engine;
pub use scene::{Scene, SceneContent, ChoiceOption};
pub use parser::{get_next_from_user_choice};
pub use engine::{load_json_scenes, render_dialogue_and_return_next, render_choices_and_return_next};
// re-exports your types so main.rs can write:
// use visual_novel::{Scene, SceneContent};
// instead of: use visual_novel::scene::{Scene, SceneContent};
