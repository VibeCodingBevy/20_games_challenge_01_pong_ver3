use bevy::prelude::Resource;
use serde::Deserialize;

#[derive(Deserialize, Resource)]
pub struct CreditsConfig {
    pub sections: Vec<CreditsSection>,
}

#[derive(Deserialize)]
pub struct CreditsSection {
    pub header: String,
    pub text: String,
}
