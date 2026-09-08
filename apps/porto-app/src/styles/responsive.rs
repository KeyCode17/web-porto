use super::theme;

pub fn responsive_css() -> String {
    format!(
        r#"
/* Tablet */
@media (max-width: 1024px) {{
    h1 {{ font-size: 4rem !important; }}
    h2 {{ font-size: 3rem !important; }}

    /* Poker cards tablet */
    .poker-card {{
        width: 140px;
        height: 200px;
        padding: 0.8rem;
    }}
    .poker-card-suit, .poker-card-suit-bottom {{
        font-size: 2rem;
    }}
    .poker-card-title {{
        font-size: 0.7rem;
    }}
}}

/* Mobile */
@media (max-width: 768px) {{
    /* Navbar: compact single row */
    h1 {{ font-size: 2.8rem !important; }}

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

    .poker-deco {{
        font-size: 4rem !important;
    }}

    .board-photo-wrap {{
        display: none !important;
    }}

    .board-strings {{
        display: none !important;
    }}

    /* Board mobile */
    .board-title {{
        font-size: 2.8rem !important;
    }}
    .board-subtitle {{
        margin-bottom: 1.5rem !important;
    }}
    .board-scene {{
        height: auto !important;
        min-height: auto !important;
        display: flex;
        flex-direction: column;
        align-items: center;
        gap: 1.5rem;
    }}
    .board-doc {{
        position: relative !important;
        width: 90% !important;
        transform: rotate(0deg) !important;
        top: auto !important;
        left: auto !important;
        padding: 1.5rem 1.2rem 1.2rem !important;
    }}
    .board-doc-title {{
        font-size: 1rem !important;
        line-height: 1.3 !important;
    }}
    .board-venue {{
        font-size: 0.6rem !important;
    }}
    .board-doc-author {{
        font-size: 0.75rem !important;
    }}
    .board-doc-tag {{
        font-size: 0.55rem !important;
    }}
    .board-stamp {{
        font-size: 0.5rem !important;
    }}
    .board-expanded {{
        top: 0;
        left: 0;
        width: 100vw;
        height: 100vh;
        border: none;
    }}
    .board-expanded-scroll {{
        padding: 1.5rem 1.2rem;
    }}
    .board-expanded-title {{
        font-size: 1.2rem !important;
    }}
    .board-expanded-venue {{
        font-size: 0.65rem !important;
    }}
    .board-expanded-author {{
        font-size: 0.8rem !important;
    }}
    .board-expanded-tag {{
        font-size: 0.6rem !important;
    }}
    .board-expanded-abstract h3 {{
        font-size: 0.85rem !important;
    }}
    .board-expanded-abstract p {{
        font-size: 0.85rem !important;
    }}
    .board-expanded-link {{
        font-size: 0.75rem !important;
        padding: 0.6rem 1rem !important;
    }}
    .board-expanded-pdf {{
        display: none;
    }}
    .board-close-btn {{
        color: {dark_brown} !important;
        border-color: {dark_brown} !important;
    }}

    /* Poker: hide fan, show stack on mobile */
    .poker-container {{
        display: none !important;
    }}
    .poker-stack {{
        display: block !important;
        position: relative;
        z-index: 1;
    }}
    .poker-card {{
        width: 80px;
        height: 120px;
        padding: 0.4rem;
        border-width: 2px;
        bottom: -10%;
    }}
    .poker-card-suit {{
        font-size: 1.2rem;
        top: 0.2rem;
        left: 0.3rem;
    }}
    .poker-card-suit-bottom {{
        font-size: 1.2rem;
        bottom: 0.2rem;
        right: 0.3rem;
    }}
    .poker-card-title {{
        font-size: 0.45rem;
        max-width: 95%;
    }}
    .poker-card-category {{
        font-size: 0.4rem;
        bottom: 1.2rem;
    }}
    .poker-card.hovered {{
        transform: rotate(var(--final-rot)) rotateX(0deg) translateY(-60px) scale(1.1);
    }}
    .poker-card.phase-dealt:active,
    .poker-card.phase-ready:active {{
        transform: rotate(var(--final-rot)) rotateX(0deg) translateY(-30px) scale(1.08);
    }}
    .poker-card-expanded {{
        flex-direction: column;
        top: 0;
        left: 0;
        width: 100vw;
        height: 100vh;
        border-radius: 0;
        border: none;
    }}
    .poker-expanded-left {{
        width: 100%;
        height: auto;
        padding: 1.2rem 1.2rem 0.8rem;
        flex-shrink: 0;
    }}
    .poker-expanded-right {{
        width: 100%;
        flex: 1;
        padding: 1.2rem;
        gap: 0.8rem;
        justify-content: flex-start;
    }}
    .poker-expanded-title {{
        font-size: 1rem;
    }}
    .poker-expanded-suit {{
        font-size: 4rem;
    }}
    .poker-expanded-desc {{
        font-size: 0.95rem;
        line-height: 1.6;
    }}
    .poker-expanded-tag {{
        font-size: 0.65rem;
        padding: 0.2rem 0.4rem;
    }}
    .poker-expanded-link {{
        font-size: 0.8rem;
        padding: 0.6rem 1rem;
    }}
    .poker-expanded-links {{
        flex-wrap: wrap;
        margin-top: 0;
    }}
    .poker-close-btn {{
        top: 0.5rem;
        right: 0.5rem;
        width: 2.5rem;
        height: 2.5rem;
        font-size: 1.2rem;
        color: {mint_white} !important;
        border-color: {mint_white} !important;
        z-index: 110;
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
        dark_brown = theme::DARK_BROWN,
    )
}
