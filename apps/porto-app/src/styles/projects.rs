use super::theme;

pub fn expanded_css() -> String {
    format!(
        r#"
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
"#,
        mint_white = theme::MINT_WHITE,
        dark_brown = theme::DARK_BROWN,
        deep_navy = theme::DEEP_NAVY,
        font_heading = theme::FONT_HEADING,
        font_mono = theme::FONT_MONO,
    )
}
