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

/* ========== Research Board (Papers) ========== */
.board-page {{
    padding: 4rem 2rem 2rem;
    min-height: 100vh;
    background: {deep_navy};
    position: relative;
    overflow: hidden;
}}

.board-title {{
    font-size: 6rem;
    font-weight: 700;
    color: {mint_white};
    text-transform: uppercase;
    text-align: center;
    margin-bottom: 0.3rem;
    font-family: {font_heading};
}}

.board-subtitle {{
    font-family: {font_mono};
    font-size: 0.9rem;
    color: {dark_brown};
    text-align: center;
    text-transform: uppercase;
    letter-spacing: 0.3em;
    margin-bottom: 3rem;
}}

.board-scene {{
    position: relative;
    width: 100%;
    max-width: 1200px;
    margin: 0 auto;
    height: 70vh;
    min-height: 500px;
}}

.board-strings {{
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    z-index: 10;
    pointer-events: none;
}}

.board-photo-wrap {{
    position: absolute;
    z-index: 1;
    cursor: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='40' height='48' viewBox='0 0 40 48'%3E%3Cdefs%3E%3ClinearGradient id='fl' x1='0' y1='0' x2='0' y2='1'%3E%3Cstop offset='0%25' stop-color='%23FFE0A0'/%3E%3Cstop offset='40%25' stop-color='%23FF9933'/%3E%3Cstop offset='100%25' stop-color='%23FF4400'/%3E%3C/linearGradient%3E%3C/defs%3E%3Cpath d='M4 26 L6 44 Q6 47 9 47 L23 47 Q26 47 26 44 L28 26Z' fill='%23B8B8B8' stroke='%23222' stroke-width='1.2' stroke-linejoin='round'/%3E%3Cpath d='M6 30 L8 44 Q8 45 10 45 L14 45' fill='none' stroke='%23D8D8D8' stroke-width='1.5' opacity='0.5'/%3E%3Crect x='7' y='20' width='18' height='7' rx='1' fill='%23E8E8E8' stroke='%23222' stroke-width='1'/%3E%3Ccircle cx='10' cy='23' r='1.5' fill='%23999' stroke='%23222' stroke-width='0.5'/%3E%3Ccircle cx='14' cy='22' r='0.8' fill='%23889'/%3E%3Ccircle cx='17' cy='23' r='0.8' fill='%23889'/%3E%3Ccircle cx='20' cy='22' r='0.8' fill='%23889'/%3E%3Cpath d='M25 24 L27 23 L35 14 Q38 11 36 9 L34 8 Q32 7 30 10 L24 20 Z' fill='%23A0A0A0' stroke='%23222' stroke-width='1' stroke-linejoin='round'/%3E%3Ccircle cx='25.5' cy='23' r='1.8' fill='%23666' stroke='%23222' stroke-width='0.6'/%3E%3Cpath d='M13 20 C11 14 9 11 10 7 C10.5 4 12.5 2 14 1 C13 5 14 8 15 10 C16 8 15.5 5 16 3 C17 5 18 8 17.5 12 C17 15 15 18 14 20' fill='url(%23fl)' opacity='0.9'/%3E%3C/svg%3E") 13 2, pointer;
}}

.board-photo-img {{
    width: 120px;
    height: 160px;
    object-fit: cover;
    border: 4px solid {mint_white};
    box-shadow: 3px 3px 15px rgba(0,0,0,0.4);
    opacity: 0.75;
    filter: grayscale(30%);
    transition: opacity 0.3s ease, filter 0.3s ease, box-shadow 0.3s ease, border-color 0.3s ease;
    will-change: opacity, filter;
}}

.board-photo-wrap:hover .board-photo-img {{
    opacity: 1;
    filter: grayscale(0%);
    box-shadow: 0 0 20px rgba(255, 120, 20, 0.6), 0 0 40px rgba(255, 60, 0, 0.3);
    border-color: #D65108;
}}

.board-pin-red {{
    background: #C0392B;
}}

.board-photo-burn {{
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    pointer-events: none;
    z-index: 3;
    overflow: hidden;
    --burn-x: 50%;
    --burn-y: 50%;
}}

.board-photo-burn::after {{
    content: '';
    position: absolute;
    left: var(--burn-x);
    top: var(--burn-y);
    width: 500px;
    height: 500px;
    transform: translate(-50%, -50%) scale(0);
    border-radius: 50%;
    background: radial-gradient(
        circle,
        rgba(0,0,0,0.95) 25%,
        rgba(50,15,0,0.9) 40%,
        rgba(180,60,0,0.8) 55%,
        rgba(255,140,0,0.5) 70%,
        rgba(255,200,50,0.3) 80%,
        transparent 100%
    );
    will-change: transform, opacity;
    animation: burn-spread 3s ease-out forwards;
}}

@keyframes burn-spread {{
    0% {{ transform: translate(-50%, -50%) scale(0); opacity: 1; }}
    40% {{ transform: translate(-50%, -50%) scale(1); opacity: 1; }}
    70% {{ transform: translate(-50%, -50%) scale(1); opacity: 0.8; }}
    100% {{ transform: translate(-50%, -50%) scale(1.1); opacity: 0; }}
}}

/* Photo crumble when burning */
.board-photo-wrap.burning .board-photo-img {{
    will-change: opacity;
    animation: photo-crumble 3s ease-in forwards;
}}

@keyframes photo-crumble {{
    0%, 30% {{ opacity: 1; }}
    45% {{ opacity: 0; }}
    100% {{ opacity: 0; }}
}}

.board-photo-wrap.burning .board-pin {{
    will-change: opacity;
    animation: pin-fade 3s ease-in forwards;
}}

@keyframes pin-fade {{
    0%, 50% {{ opacity: 1; }}
    100% {{ opacity: 0; }}
}}

@keyframes chat-loading {{
    0% {{ left: -40%; }}
    100% {{ left: 100%; }}
}}

.board-doc {{
    position: absolute;
    width: 320px;
    background: {mint_white};
    padding: 2rem 1.8rem 1.5rem;
    cursor: pointer;
    transition: transform 0.3s ease, box-shadow 0.3s ease, filter 0.3s ease;
    box-shadow: 4px 4px 20px rgba(0,0,0,0.4);
    border: none;
    z-index: 2;
}}

.board-doc:hover {{
    transform: rotate(0deg) scale(1.03) !important;
    box-shadow: 8px 8px 30px rgba(0,0,0,0.5);
    z-index: 10;
}}

.board-doc-blurred {{
    filter: blur(3px);
    pointer-events: none;
}}

.board-pin {{
    position: absolute;
    top: -8px;
    left: 50%;
    transform: translateX(-50%);
    width: 20px;
    height: 20px;
    background: {dark_brown};
    border-radius: 50%;
    box-shadow: 0 2px 4px rgba(0,0,0,0.3);
    z-index: 5;
}}

.board-pin::after {{
    content: '';
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    width: 8px;
    height: 8px;
    background: rgba(255,255,255,0.4);
    border-radius: 50%;
}}

.board-stamp {{
    position: absolute;
    top: 1.2rem;
    right: 1rem;
    font-family: {font_mono};
    font-size: 0.6rem;
    font-weight: 700;
    color: #C0392B;
    border: 2px solid #C0392B;
    padding: 0.15rem 0.4rem;
    text-transform: uppercase;
    transform: rotate(8deg);
    opacity: 0.8;
    letter-spacing: 0.1em;
}}

.board-venue {{
    font-family: {font_mono};
    font-size: 0.7rem;
    color: #888;
    text-transform: uppercase;
    margin-bottom: 0.8rem;
}}

.board-doc-title {{
    font-size: 1.15rem;
    font-weight: 700;
    color: {deep_navy};
    line-height: 1.3;
    margin-bottom: 0.5rem;
}}

.board-doc-author {{
    font-size: 0.8rem;
    color: {dark_brown};
    margin-bottom: 0.8rem;
}}

.board-doc-tags {{
    display: flex;
    flex-wrap: wrap;
    gap: 0.3rem;
}}

.board-doc-tag {{
    font-family: {font_mono};
    font-size: 0.6rem;
    border: 1px solid {deep_navy};
    padding: 0.15rem 0.4rem;
    color: {deep_navy};
}}

/* Board expanded overlay */
.board-overlay {{
    position: fixed;
    top: 0;
    left: 0;
    width: 100vw;
    height: 100vh;
    background: rgba(0, 0, 0, 0.6);
    z-index: 99;
    cursor: pointer;
}}

.board-expanded {{
    position: fixed;
    top: 3vh;
    left: 5vw;
    width: 90vw;
    height: 94vh;
    background: {mint_white};
    z-index: 100;
    border: 4px solid {deep_navy};
    animation: card-expand 0.4s cubic-bezier(0.34, 1.56, 0.64, 1) forwards;
    display: flex;
    flex-direction: column;
}}

.board-expanded-scroll {{
    flex: 1;
    overflow-y: auto;
    padding: 2.5rem 3rem;
}}

.board-expanded-venue {{
    font-family: {font_mono};
    font-size: 0.8rem;
    color: #888;
    text-transform: uppercase;
    margin-bottom: 0.5rem;
}}

.board-expanded-title {{
    font-size: 2rem;
    font-weight: 700;
    color: {deep_navy};
    line-height: 1.3;
    margin-bottom: 0.5rem;
}}

.board-expanded-author {{
    font-size: 0.95rem;
    color: {dark_brown};
    margin-bottom: 1rem;
}}

.board-expanded-tags {{
    display: flex;
    flex-wrap: wrap;
    gap: 0.4rem;
    margin-bottom: 1.5rem;
}}

.board-expanded-tag {{
    font-family: {font_mono};
    font-size: 0.7rem;
    border: 2px solid {deep_navy};
    padding: 0.2rem 0.5rem;
    color: {deep_navy};
}}

.board-expanded-abstract {{
    background: {deep_navy};
    padding: 1.5rem;
    margin-bottom: 1.5rem;
}}

.board-expanded-abstract h3 {{
    font-family: {font_mono};
    font-size: 1rem;
    font-weight: 700;
    color: {dark_brown};
    text-transform: uppercase;
    margin-bottom: 0.8rem;
}}

.board-expanded-abstract p {{
    font-size: 1rem;
    line-height: 1.7;
    color: {mint_white};
}}

.board-expanded-links {{
    display: flex;
    gap: 1rem;
    flex-wrap: wrap;
    margin-bottom: 1.5rem;
}}

.board-expanded-link {{
    font-family: {font_mono};
    font-weight: 700;
    font-size: 0.9rem;
    padding: 0.8rem 1.5rem;
    text-transform: uppercase;
    text-decoration: none;
    transition: opacity 0.2s;
}}

.board-expanded-link:hover {{
    opacity: 0.85;
}}

.board-link-primary {{
    color: {mint_white};
    background: {deep_navy};
    border: 3px solid {deep_navy};
}}

.board-link-secondary {{
    color: {deep_navy};
    background: none;
    border: 3px solid {deep_navy};
}}

.board-expanded-pdf {{
    border: 3px solid {deep_navy};
    margin-top: 0.5rem;
}}

.board-close-btn {{
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

.board-close-btn:hover {{
    background: rgba(214, 81, 8, 0.1);
}}


/* Mobile card stack - hidden on desktop */
.poker-stack {{
    display: none;
}}

.poker-stack-cards {{
    position: relative;
    width: 220px;
    height: 320px;
    margin: 0 auto;
}}

.poker-stack-card {{
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    border: 3px solid {mint_white};
    border-radius: 10px;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 1.5rem;
    cursor: pointer;
    transition: transform 0.25s cubic-bezier(0.4, 0, 0.2, 1),
                opacity 0.25s ease,
                scale 0.25s ease;
    box-shadow: 0 4px 20px rgba(0,0,0,0.3);
}}

.poker-stack-title {{
    font-family: {font_mono};
    font-size: 1.1rem;
    font-weight: 700;
    text-align: center;
    color: {mint_white};
    text-transform: uppercase;
    line-height: 1.3;
}}

.poker-stack-nav {{
    font-family: {font_mono};
    font-size: 1.5rem;
    color: {mint_white};
    background: none;
    border: 2px solid {mint_white};
    width: 3rem;
    height: 3rem;
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    border-radius: 4px;
    transition: background 0.2s;
}}

.poker-stack-nav:hover {{
    background: rgba(229, 229, 229, 0.1);
}}

.poker-stack-nav:disabled {{
    opacity: 0.3;
    cursor: default;
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
        deep_navy = theme::DEEP_NAVY,
        font_heading = theme::FONT_HEADING,
        font_mono = theme::FONT_MONO,
        dark_brown = theme::DARK_BROWN,
    )
}
