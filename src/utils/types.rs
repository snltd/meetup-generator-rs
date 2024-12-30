use serde::{Deserialize, Serialize};

#[derive(Deserialize, Clone, Debug)]
pub struct AllTheThings {
    pub adjective: Vec<String>,
    pub company: Vec<String>,
    pub driver: Vec<String>,
    pub extreme: Vec<String>,
    pub first_name: Vec<String>,
    pub degree: Vec<String>,
    pub job_role: Vec<String>,
    pub job_title: Vec<String>,
    pub language: Vec<String>,
    pub food: Vec<String>,
    pub food_style: Vec<String>,
    pub last_name: Vec<String>,
    pub panacea: Vec<String>,
    pub quantifier: Vec<String>,
    pub service: Vec<String>,
    pub skill_level: Vec<String>,
    pub something_ops: Vec<String>,
    pub tech: Vec<String>,
    pub template: Vec<String>,
    pub time: Vec<String>,
    pub verb: Vec<String>,
}

#[derive(Clone)]
pub struct Meetup {
    pub things: AllTheThings,
    pub words: Words,
}

pub type Words = Vec<String>;

#[derive(Deserialize, Serialize)]
pub struct Agenda {
    pub talks: Vec<Talk>,
    pub refreshment: String,
    pub location: String,
    pub date: String,
}

#[derive(Deserialize, Debug, PartialEq, Serialize)]
pub struct Talk {
    pub title: String,
    pub talker: String,
    pub role: String,
    pub company: String,
}
