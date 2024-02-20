use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub enum ComponentSize{
	ExtraSmall,
	Small,
	Base, //Default
	Large
}

impl Default for ComponentSize {
    fn default() -> Self {
        ComponentSize::Base
    }
}

pub enum ComponentType{
	Info,
	Success,
	Neutral, //Default
	Warning,
	Error
}

impl Default for ComponentType {
    fn default() -> Self {
        ComponentType::Neutral
    }
}