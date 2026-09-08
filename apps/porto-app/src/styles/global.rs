use super::{base, papers, projects, responsive};

pub fn global_css() -> String {
    [
        base::base_css(),
        projects::fan_css(),
        papers::papers_css(),
        projects::stack_css(),
        responsive::responsive_css(),
    ]
    .join("\n")
}
