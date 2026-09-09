use super::theme;

pub fn belt_css() -> String {
    format!(
        r#"
.range-scroll {{
    position: relative;
}}

.range-stage {{
    position: sticky;
    top: 0;
    height: 100vh;
    overflow: hidden;
    display: flex;
    flex-direction: column;
    justify-content: flex-end;
    background: {deep_navy};
}}

.range-canvas {{
    position: absolute;
    inset: 0;
    width: 100%;
    height: 100%;
    pointer-events: none;
}}

.range-head {{
    position: absolute;
    top: 5.5rem;
    left: 0;
    right: 0;
    text-align: center;
    z-index: 2;
    pointer-events: none;
}}

.range-heading {{
    font-family: {font_heading};
    font-size: 4.6rem;
    font-weight: 700;
    line-height: 1;
    letter-spacing: -0.02em;
    text-transform: uppercase;
    color: {mint_white};
    margin: 0;
}}

.range-sub {{
    font-family: {font_mono};
    font-size: 0.78rem;
    letter-spacing: 0.28em;
    text-transform: uppercase;
    color: {muted_teal};
    margin-top: 0.5rem;
}}

.range-hud {{
    position: absolute;
    top: 13rem;
    left: 2rem;
    display: flex;
    flex-direction: column;
    gap: 0.35rem;
    z-index: 2;
}}

.range-hud-line {{
    font-family: {font_mono};
    font-size: 0.72rem;
    letter-spacing: 0.22em;
    text-transform: uppercase;
    color: {dark_brown};
    border-left: 4px solid {dark_brown};
    padding-left: 0.6rem;
}}

.range-readout {{
    position: absolute;
    right: 2.5rem;
    bottom: 12rem;
    z-index: 2;
    width: min(42vw, 520px);
    border-left: 8px solid {dark_brown};
    padding-left: 1.4rem;
}}

.range-title {{
    font-family: {font_heading};
    font-size: 2.7rem;
    font-weight: 700;
    line-height: 1.0;
    letter-spacing: -0.01em;
    text-transform: uppercase;
    color: {mint_white};
    margin-bottom: 0.7rem;
}}

.range-desc {{
    font-size: 1rem;
    line-height: 1.55;
    color: {muted_teal};
    margin-bottom: 0.9rem;
}}

.range-tags {{
    display: flex;
    flex-wrap: wrap;
    gap: 0.35rem;
    margin-bottom: 1.1rem;
}}

.range-tag {{
    font-family: {font_mono};
    font-size: 0.66rem;
    letter-spacing: 0.1em;
    text-transform: uppercase;
    color: {mint_white};
    border: 2px solid rgba(229, 229, 229, 0.3);
    padding: 0.18rem 0.5rem;
}}

.range-open {{
    font-family: {font_mono};
    font-size: 0.78rem;
    letter-spacing: 0.18em;
    text-transform: uppercase;
    color: {deep_navy};
    background: {mint_white};
    border: none;
    padding: 0.7rem 1.4rem;
    cursor: pointer;
    transition: background 0.15s ease, color 0.15s ease;
}}

.range-open:hover {{
    background: {dark_brown};
    color: {mint_white};
}}

.range-peek {{
    position: absolute;
    left: 50%;
    bottom: 10.5rem;
    z-index: 3;
    width: max-content;
    max-width: 340px;
    background: {deep_navy};
    border: 2px solid {mint_white};
    padding: 0.6rem 0.8rem;
    pointer-events: none;
    display: flex;
    flex-direction: column;
    gap: 0.3rem;
}}

.range-peek-title {{
    font-family: {font_heading};
    font-size: 0.92rem;
    font-weight: 700;
    letter-spacing: 0.04em;
    text-transform: uppercase;
    color: {mint_white};
}}

.range-peek-tech {{
    font-family: {font_mono};
    font-size: 0.64rem;
    letter-spacing: 0.08em;
    line-height: 1.5;
    color: {muted_teal};
}}

.belt-rail {{
    position: absolute;
    left: 0;
    right: 0;
    bottom: 1.6rem;
    z-index: 2;
    display: flex;
    justify-content: center;
    align-items: flex-end;
    overflow: hidden;
    pointer-events: none;
}}

.belt-rail .belt-round {{
    pointer-events: auto;
}}

.belt-strip {{
    display: flex;
    align-items: flex-end;
    gap: 12px;
    transform-origin: center bottom;
    transition: transform 0.42s cubic-bezier(0.2, 0.8, 0.2, 1);
    will-change: transform;
}}

.belt-round {{
    flex: 0 0 auto;
    width: 42px;
    height: 136px;
    padding: 0;
    background: transparent;
    border: none;
    cursor: pointer;
    display: block;
    opacity: 0;
    transform: translateY(28px);
    transition: transform 0.45s cubic-bezier(0.34, 1.4, 0.64, 1), opacity 0.35s ease;
    transition-delay: var(--load-delay, 0ms);
}}

.belt-round.loaded {{
    opacity: 0.62;
    transform: translateY(0);
}}

.belt-round.chambered {{
    opacity: 1;
    transform: translateY(-22px);
    filter: brightness(1.2);
}}

.belt-round:focus-visible {{
    outline: 2px solid {mint_white};
    outline-offset: 3px;
}}

.belt-round-shape {{
    width: 100%;
    height: 100%;
    display: block;
}}

.belt-round-bullet {{
    fill: {copper};
}}

.belt-round-case {{
    fill: {brass};
}}

.belt-round-shine {{
    fill: rgba(255, 255, 255, 0.28);
}}

/* Fired: the bullet is gone and only the blackened case is left on the belt.
   Driven purely by position relative to the chamber, so scrolling back up
   refills them without any state to unwind. */
.belt-round.spent .belt-round-bullet {{
    opacity: 0;
}}

.belt-round.spent .belt-round-case {{
    fill: #6E5B27;
}}

.belt-round.spent .belt-round-shine {{
    opacity: 0.35;
}}

.belt-round.spent {{
    opacity: 0.4;
}}

.belt-round-bullet,
.belt-round-case,
.belt-round-shine {{
    transition: opacity 0.25s ease, fill 0.25s ease;
}}
"#,
        mint_white = theme::MINT_WHITE,
        muted_teal = theme::MUTED_TEAL,
        deep_navy = theme::DEEP_NAVY,
        dark_brown = theme::DARK_BROWN,
        font_heading = theme::FONT_HEADING,
        font_mono = theme::FONT_MONO,
        brass = "#C8A951",
        copper = "#B87333",
    )
}
