use super::{base, belt, papers, projects, responsive};

pub fn global_css() -> String {
    [
        base::base_css(),
        belt::belt_css(),
        papers::papers_css(),
        projects::expanded_css(),
        responsive::responsive_css(),
    ]
    .join("\n")
}
