use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone)]
pub struct About {
    pub name: String,
    pub title: String,
    pub narrative: String,
    pub facts: Vec<Fact>,
    pub social_links: Vec<SocialLink>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Fact {
    pub label: String,
    pub value: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SocialLink {
    pub platform: String,
    pub url: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Project {
    pub slug: String,
    pub title: String,
    pub short_description: String,
    pub long_description: String,
    pub tech_stack: Vec<String>,
    pub category: String,
    pub repo_url: String,
    pub demo_url: String,
    pub images: Vec<String>,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Experience {
    pub company: String,
    pub role: String,
    pub start_date: String,
    pub end_date: String,
    pub summary: String,
    pub details: String,
    pub tech: Vec<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Paper {
    pub slug: String,
    pub title: String,
    pub authors: Vec<String>,
    pub venue: String,
    pub r#abstract: String,
    pub pdf_file: String,
    pub tags: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Skill {
    pub name: String,
    pub category: String,
    pub proficiency: u8,
    pub connections: Vec<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ChatConfig {
    pub system_prompt: String,
}

// Top-level TOML wrappers
#[derive(Debug, Deserialize)]
pub struct AboutFile {
    pub about: About,
}

#[derive(Debug, Deserialize)]
pub struct ProjectsFile {
    pub project: Vec<Project>,
}

#[derive(Debug, Deserialize)]
pub struct ExperienceFile {
    pub experience: Vec<Experience>,
}

#[derive(Debug, Deserialize)]
pub struct PapersFile {
    pub paper: Vec<Paper>,
}

#[derive(Debug, Deserialize)]
pub struct SkillsFile {
    pub skill: Vec<Skill>,
}

#[derive(Debug, Deserialize)]
pub struct ChatFile {
    pub chat: ChatConfig,
}
