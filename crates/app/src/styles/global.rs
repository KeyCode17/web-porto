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

/* Poker card fan */
.poker-container {{
    position: relative;
    width: 100%;
    height: 65vh;
    display: flex;
    align-items: flex-end;
    justify-content: center;
    perspective: 1200px;
    overflow: visible;
}}

.poker-card {{
    position: absolute;
    bottom: -25%;
    width: 180px;
    height: 260px;
    border: 4px solid {mint_white};
    border-radius: 8px;
    cursor: pointer;
    transform-origin: center 120%;
    transition: transform 0.6s cubic-bezier(0.34, 1.56, 0.64, 1),
                opacity 0.4s ease,
                filter 0.3s ease,
                box-shadow 0.2s ease;
    opacity: 0;
    transform: rotate(0deg) rotateX(5deg) scale(0.8);
    backface-visibility: hidden;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 1.2rem 1rem;
    box-shadow: 0 4px 20px rgba(0,0,0,0.3);
    overflow: hidden;
    z-index: 1;
}}

.poker-card.phase-shuffle {{
    opacity: 1;
    transform: rotate(var(--shuffle-rot)) translateX(var(--shuffle-x)) rotateX(5deg) scale(0.8);
}}

.poker-card.phase-dealt {{
    opacity: 1;
    transform: rotate(var(--final-rot)) rotateX(5deg);
    transition-delay: var(--deal-delay);
}}

.poker-card.phase-ready {{
    transition: transform 0.3s cubic-bezier(0.25, 0.46, 0.45, 0.94),
                filter 0.3s ease,
                box-shadow 0.3s ease;
    transition-delay: 0ms;
}}

.poker-card.hovered {{
    transform: rotate(var(--final-rot)) rotateX(0deg) translateY(-150px) scale(1.15);
    box-shadow: 0 12px 40px rgba(0,0,0,0.5);
    z-index: 50;
    transition-delay: 0ms;
}}

.poker-card.blurred {{
    filter: blur(4px);
    pointer-events: none;
}}

.poker-card-suit {{
    font-size: 2.5rem;
    position: absolute;
    top: 0.5rem;
    left: 0.8rem;
    color: var(--suit-color, {mint_white});
}}

.poker-card-suit-bottom {{
    font-size: 2.5rem;
    position: absolute;
    bottom: 0.5rem;
    right: 0.8rem;
    transform: rotate(180deg);
    color: var(--suit-color, {mint_white});
}}

.poker-card-title {{
    font-family: {font_mono};
    font-size: 0.85rem;
    font-weight: 700;
    text-align: center;
    color: {mint_white};
    text-transform: uppercase;
    line-height: 1.3;
    max-width: 90%;
}}

.poker-card-category {{
    font-family: {font_mono};
    font-size: 0.65rem;
    text-transform: uppercase;
    color: rgba(229, 229, 229, 0.7);
    position: absolute;
    bottom: 2.5rem;
}}

/* Expanded card overlay */
.poker-overlay {{
    position: fixed;
    top: 0;
    left: 0;
    width: 100vw;
    height: 100vh;
    background: rgba(0, 0, 0, 0.5);
    z-index: 99;
    cursor: pointer;
}}

.poker-card-expanded {{
    position: fixed;
    top: 5vh;
    left: 5vw;
    width: 90vw;
    height: 85vh;
    border: 4px solid {mint_white};
    border-radius: 12px;
    z-index: 100;
    display: flex;
    flex-direction: row;
    overflow: hidden;
    animation: card-expand 0.5s cubic-bezier(0.34, 1.56, 0.64, 1) forwards;
}}

@keyframes card-expand {{
    0% {{
        opacity: 0;
        transform: scale(0.3) rotate(-5deg);
    }}
    100% {{
        opacity: 1;
        transform: scale(1) rotate(0deg);
    }}
}}

.poker-expanded-left {{
    width: 40%;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 2rem;
    position: relative;
}}

.poker-expanded-suit {{
    font-size: 8rem;
    opacity: 0.15;
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
}}

.poker-expanded-title {{
    font-family: {font_heading};
    font-size: 2.5rem;
    font-weight: 700;
    color: {mint_white};
    text-transform: uppercase;
    text-align: center;
    z-index: 1;
    line-height: 1.2;
}}

.poker-expanded-category-label {{
    font-family: {font_mono};
    font-size: 0.85rem;
    color: rgba(229, 229, 229, 0.7);
    text-transform: uppercase;
    margin-top: 1rem;
    z-index: 1;
}}

.poker-expanded-right {{
    width: 60%;
    background: {mint_white};
    padding: 3rem;
    overflow-y: auto;
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
}}

.poker-expanded-desc {{
    font-size: 1.1rem;
    line-height: 1.8;
    color: {deep_navy};
}}

.poker-expanded-tags {{
    display: flex;
    flex-wrap: wrap;
    gap: 0.5rem;
}}

.poker-expanded-tag {{
    font-family: {font_mono};
    font-size: 0.75rem;
    border: 2px solid {deep_navy};
    padding: 0.3rem 0.6rem;
    color: {deep_navy};
}}

.poker-expanded-links {{
    display: flex;
    gap: 1rem;
    margin-top: auto;
}}

.poker-expanded-link {{
    font-family: {font_mono};
    font-weight: 700;
    font-size: 0.9rem;
    padding: 0.8rem 1.5rem;
    text-transform: uppercase;
    text-decoration: none;
}}

.poker-close-btn {{
    position: absolute;
    top: 1rem;
    right: 1rem;
    font-family: {font_mono};
    font-size: 1.5rem;
    color: {dark_brown};
    background: none;
    border: 2px solid {dark_brown};
    width: 2.5rem;
    height: 2.5rem;
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    z-index: 101;
    transition: background 0.2s;
}}

.poker-close-btn:hover {{
    background: rgba(229, 229, 229, 0.2);
}}

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

    /* Poker cards mobile */
    .poker-container {{
        height: 60vh;
    }}
    .poker-card {{
        width: 100px;
        height: 150px;
        padding: 0.6rem;
        border-width: 3px;
    }}
    .poker-card-suit {{
        font-size: 1.5rem;
        top: 0.3rem;
        left: 0.4rem;
    }}
    .poker-card-suit-bottom {{
        font-size: 1.5rem;
        bottom: 0.3rem;
        right: 0.4rem;
    }}
    .poker-card-title {{
        font-size: 0.55rem;
    }}
    .poker-card-category {{
        font-size: 0.45rem;
        bottom: 1.5rem;
    }}
    .poker-card-expanded {{
        flex-direction: column;
        top: 2vh;
        left: 2vw;
        width: 96vw;
        height: 92vh;
    }}
    .poker-expanded-left {{
        width: 100%;
        height: 30%;
        padding: 1.5rem;
    }}
    .poker-expanded-right {{
        width: 100%;
        height: 70%;
        padding: 1.5rem;
    }}
    .poker-expanded-title {{
        font-size: 1.5rem;
    }}
    .poker-expanded-suit {{
        font-size: 5rem;
    }}
    .poker-card.phase-dealt:active {{
        transform: rotate(var(--final-rot)) rotateX(5deg) translateY(-15px) scale(1.05);
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
        font_mono = theme::FONT_MONO,
        dark_brown = theme::DARK_BROWN,
    )
}
