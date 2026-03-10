use porto_shared::*;

const ABOUT_TOML: &str = include_str!("../../content/about.toml");
const PROJECTS_TOML: &str = include_str!("../../content/projects.toml");
const EXPERIENCE_TOML: &str = include_str!("../../content/experience.toml");
const PAPERS_TOML: &str = include_str!("../../content/papers.toml");
const SKILLS_TOML: &str = include_str!("../../content/skills.toml");
const FAQ_TOML: &str = include_str!("../../content/faq.toml");

pub fn load_about() -> About {
    toml::from_str::<AboutFile>(ABOUT_TOML)
        .expect("Invalid about.toml")
        .about
}

pub fn load_projects() -> Vec<Project> {
    toml::from_str::<ProjectsFile>(PROJECTS_TOML)
        .expect("Invalid projects.toml")
        .project
}

pub fn load_experience() -> Vec<Experience> {
    toml::from_str::<ExperienceFile>(EXPERIENCE_TOML)
        .expect("Invalid experience.toml")
        .experience
}

pub fn load_papers() -> Vec<Paper> {
    toml::from_str::<PapersFile>(PAPERS_TOML)
        .expect("Invalid papers.toml")
        .paper
}

pub fn load_skills() -> Vec<Skill> {
    toml::from_str::<SkillsFile>(SKILLS_TOML)
        .expect("Invalid skills.toml")
        .skill
}

pub fn load_faq() -> Vec<FaqEntry> {
    toml::from_str::<FaqFile>(FAQ_TOML)
        .expect("Invalid faq.toml")
        .faq
}
