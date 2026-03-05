use super::theme;

pub fn global_css() -> String {
    format!(
        r#"
@import url('https://fonts.googleapis.com/css2?family=Space+Grotesk:wght@300;400;500;600;700&display=swap');
@import url('https://fonts.googleapis.com/css2?family=JetBrains+Mono:wght@400;500;700&display=swap');

* {{
    margin: 0;
    padding: 0;
    box-sizing: border-box;
}}

html {{
    scroll-behavior: smooth;
}}

body {{
    background-color: {mint_white};
    color: {deep_navy};
    font-family: {font_heading};
    overflow-x: hidden;
}}

a {{
    color: {deep_navy};
    text-decoration: none;
}}

a:hover {{
    color: {dark_brown};
}}

[data-reveal] {{
    opacity: 0;
    transform: translateY(30px);
    transition: opacity 0.6s ease, transform 0.6s ease;
}}

[data-reveal].revealed {{
    opacity: 1;
    transform: translateY(0);
}}

@media (hover: hover) {{
    * {{ cursor: none !important; }}
}}

@media (hover: none) {{
    .custom-cursor {{ display: none; }}
}}

/* Tablet */
@media (max-width: 1024px) {{
    h1 {{ font-size: 4rem !important; }}
    h2 {{ font-size: 3rem !important; }}
}}

/* Mobile */
@media (max-width: 768px) {{
    h1 {{ font-size: 2.5rem !important; }}
    h2 {{ font-size: 2rem !important; }}

    /* Stack navbar vertically */
    .nav-links {{
        flex-direction: column;
        gap: 0.5rem !important;
        align-items: flex-end;
    }}

    /* Reduce section padding */
    section {{
        padding: 3rem 1rem !important;
    }}

    /* Skills canvas smaller */
    #skills-canvas {{
        height: 400px !important;
    }}

    /* Timeline: disable scroll hijack on mobile */
    #timeline-scroll-container {{
        height: auto !important;
    }}

    #timeline-track {{
        flex-direction: column !important;
        transform: none !important;
        padding: 2rem 1rem !important;
        gap: 1.5rem !important;
    }}

    #timeline-track > div {{
        min-width: auto !important;
        max-width: 100% !important;
    }}
}}

/* Small mobile */
@media (max-width: 480px) {{
    h1 {{ font-size: 2rem !important; }}
}}
"#,
        mint_white = theme::MINT_WHITE,
        deep_navy = theme::DEEP_NAVY,
        font_heading = theme::FONT_HEADING,
        dark_brown = theme::DARK_BROWN,
    )
}
