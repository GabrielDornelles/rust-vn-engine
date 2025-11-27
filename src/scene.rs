use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Scene {
    pub scene_id: String,
    pub content: SceneContent,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum SceneContent {
    Dialogue {
        speaker: String,
        text: String,
        next: Option<String>, // last dialogue wont have next (will be null)
    },
    Choice {
        speaker: String,
        text: String,
        options: Vec<ChoiceOption>,
    },
}

#[derive(Debug, Deserialize)]
pub struct ChoiceOption {
    pub description: String,
    pub next: String,
}