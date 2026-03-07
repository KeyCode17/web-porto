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

.custom-cursor {{ display: none; }}

/* Timeline: desktop/mobile toggle */
.timeline-mobile {{
    display: none;
}}

/* Tablet */
@media (max-width: 1024px) {{
    h1 {{ font-size: 4rem !important; }}
    h2 {{ font-size: 3rem !important; }}
}}

/* Mobile */
@media (max-width: 768px) {{
    /* Navbar: compact single row */
    nav {{
        padding: 0.5rem 0.8rem !important;
        flex-direction: row !important;
        flex-wrap: nowrap !important;
        align-items: center !important;
    }}
    nav > a span {{
        font-size: 0.85rem !important;
    }}
    .nav-links {{
        flex-direction: row !important;
        gap: 0.6rem !important;
    }}
    .nav-links a span {{
        font-size: 0.7rem !important;
    }}

    /* Hero text */
    #hero-name {{
        font-size: 3.5rem !important;
    }}
    #hero-subtitle {{
        font-size: 1rem !important;
    }}

    /* About content */
    #about-content {{
        padding: 3.5rem 1rem 6rem 1rem !important;
    }}
    #about-heading {{
        font-size: 2.5rem !important;
    }}
    #about-narrative {{
        font-size: 1rem !important;
        margin-bottom: 1.5rem !important;
    }}
    .about-fact-value {{
        font-size: 1.3rem !important;
    }}
    .about-fact-label {{
        font-size: 0.75rem !important;
    }}
    .about-fact-card {{
        padding: 1rem !important;
    }}

    /* Reduce section padding */
    section {{
        padding: 3rem 1rem !important;
    }}

    /* Skills */
    #skills {{
        min-height: auto !important;
    }}
    #skills-canvas {{
        height: 350px !important;
    }}

    /* Timeline: vertical on mobile */
    .timeline-desktop {{
        display: none !important;
    }}
    .timeline-mobile {{
        display: block !important;
    }}

    /* Contact links */
    #contact a {{
        font-size: 1rem !important;
        padding: 0.7rem 1.2rem !important;
    }}
    #contact > div > div {{
        gap: 1.5rem !important;
    }}
}}

/* Small mobile */
@media (max-width: 480px) {{
    #hero-name {{
        font-size: 2.5rem !important;
    }}
    #hero-subtitle {{
        font-size: 0.85rem !important;
    }}
    #about-heading {{
        font-size: 2rem !important;
    }}
    #about-narrative {{
        font-size: 0.9rem !important;
    }}
    .about-fact-value {{
        font-size: 1.1rem !important;
    }}
    .nav-links {{
        gap: 0.4rem !important;
    }}
    .nav-links a span {{
        font-size: 0.6rem !important;
    }}
    nav > a span {{
        font-size: 0.75rem !important;
    }}
}}
"#,
        mint_white = theme::MINT_WHITE,
        deep_navy = theme::DEEP_NAVY,
        font_heading = theme::FONT_HEADING,
        dark_brown = theme::DARK_BROWN,
    )
}
