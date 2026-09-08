pub mod chart_input;
pub mod constants;
pub mod dates;
pub mod scroll_hijack;
pub mod track;

use self::chart_input::setup_chart_click;
use self::constants::{
    BAR_FOCUS_EVENT, FOCUSED_ATTRIBUTE, TIMELINE_CHART_ID, TIMELINE_CONTAINER_ID,
    TIMELINE_CURSOR_ID, TIMELINE_TRACK_ID,
};
use self::dates::{bar_color, date_to_months, format_date_display, parse_date};
use self::scroll_hijack::setup_scroll_hijack;
use crate::data;
use crate::libs::EventListener;
use crate::styles::theme;
use dioxus::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

#[component]
pub fn Timeline() -> Element {
    let experiences = data::load_experience();
    let mut selected = use_signal(|| None::<usize>);

    let mut min_months = f64::MAX;
    let mut max_months = f64::MIN;
    for exp in experiences.iter() {
        let (sy, sm) = parse_date(&exp.start_date);
        let (ey, em) = parse_date(&exp.end_date);
        let start = date_to_months(sy, sm);
        let end = date_to_months(ey, em);
        if start < min_months {
            min_months = start;
        }
        if end > max_months {
            max_months = end;
        }
    }

    min_months -= 2.0;
    max_months += 2.0;
    let total_months = max_months - min_months;

    let start_year = (min_months / 12.0).floor() as i32;
    let end_year = (max_months / 12.0).ceil() as i32;

    let mut row_ends: Vec<f64> = Vec::new();
    let mut row_assignments: Vec<usize> = Vec::new();

    for exp in experiences.iter() {
        let (sy, sm) = parse_date(&exp.start_date);
        let (ey, em) = parse_date(&exp.end_date);
        let start = date_to_months(sy, sm);
        let end = date_to_months(ey, em);

        let existing_row = row_ends.iter_mut().position(|row_end| {
            let fits = start >= *row_end + 1.0;
            if fits {
                *row_end = end;
            }
            fits
        });

        let assigned_row = match existing_row {
            Some(row) => row,
            None => {
                row_ends.push(end);
                row_ends.len() - 1
            }
        };
        row_assignments.push(assigned_row);
    }

    let num_rows = row_ends.len();
    let row_height: usize = 48;
    let bar_height: usize = 38;
    let axis_height: usize = 36;
    let _chart_height = num_rows * row_height + axis_height + 20;

    let track_width: usize = 2000;
    let hijack_height: usize = track_width * 2;

    let listeners: Rc<RefCell<Vec<EventListener>>> = use_hook(|| Rc::new(RefCell::new(Vec::new())));

    use_effect({
        let listeners = listeners.clone();
        move || {
            let mut owned = listeners.borrow_mut();
            if !owned.is_empty() {
                return;
            }

            owned.extend(setup_scroll_hijack(
                TIMELINE_CONTAINER_ID,
                TIMELINE_TRACK_ID,
            ));
            owned.extend(setup_chart_click(
                TIMELINE_CONTAINER_ID,
                TIMELINE_CHART_ID,
                TIMELINE_TRACK_ID,
            ));

            let container = web_sys::window()
                .and_then(|window| window.document())
                .and_then(|document| document.get_element_by_id(TIMELINE_CONTAINER_ID));

            if let Some(container) = container {
                owned.extend(EventListener::new(
                    container.as_ref(),
                    BAR_FOCUS_EVENT,
                    move |_| {
                        let focused = web_sys::window()
                            .and_then(|window| window.document())
                            .and_then(|document| document.get_element_by_id(TIMELINE_CONTAINER_ID))
                            .and_then(|cont| cont.get_attribute(FOCUSED_ATTRIBUTE))
                            .and_then(|idx| idx.parse::<usize>().ok());

                        if let Some(idx) = focused {
                            selected.set(Some(idx));
                        }
                    },
                ));
            }
        }
    });

    let mut sorted_indices: Vec<usize> = (0..experiences.len()).collect();
    sorted_indices.sort_by(|&a, &b| {
        let (ay, am) = parse_date(&experiences[b].start_date);
        let (by, bm) = parse_date(&experiences[a].start_date);
        (by, bm).cmp(&(ay, am))
    });

    rsx! {
        section { id: "experience",
            style: "background-color: {theme::DEEP_NAVY};",

            div {
                class: "timeline-desktop",
                id: TIMELINE_CONTAINER_ID,
                style: "height: {hijack_height}px; position: relative;",

                div {
                    style: "position: sticky; top: 0; height: 100vh; overflow: hidden; display: flex; flex-direction: column; background-color: {theme::DEEP_NAVY}; padding: 60px 2rem 2rem 2rem;",

                    h2 {
                        style: "font-size: 4rem; font-weight: 700; color: {theme::DARK_BROWN}; text-transform: uppercase; margin-bottom: 1rem; flex-shrink: 0;",
                        "EXPERIENCE & EDUCATION"
                    }

                    div {
                        id: TIMELINE_CHART_ID,
                        style: "border: 3px solid {theme::DEEP_NAVY}; background: {theme::MINT_WHITE}; overflow: hidden; position: relative; flex: 1 1 auto; min-height: 0;",

                        div {
                            id: TIMELINE_CURSOR_ID,
                            style: "position: absolute; top: 0; bottom: 0; width: 2px; background: {theme::DARK_BROWN}; z-index: 50; left: 0; pointer-events: none; transition: none;",
                            div {
                                style: "position: absolute; bottom: 30px; left: 50%; transform: translateX(-50%); width: 0; height: 0; border-left: 6px solid transparent; border-right: 6px solid transparent; border-top: 8px solid {theme::DARK_BROWN};",
                            }
                        }

                        div {
                            id: TIMELINE_TRACK_ID,
                            style: "position: relative; width: {track_width}px; height: 100%; padding: 0.75rem 0; will-change: transform;",

                            for year in start_year..=end_year {
                                {
                                    let x_pos = (date_to_months(year, 1) - min_months) / total_months * 100.0;
                                    rsx! {
                                        div {
                                            style: "position: absolute; left: {x_pos}%; top: 0; bottom: {axis_height}px; width: 1px; background: rgba(2, 24, 43, 0.15); z-index: 1;",
                                        }
                                        div {
                                            style: "position: absolute; left: {x_pos}%; bottom: 6px; transform: translateX(-50%); font-family: {theme::FONT_MONO}; font-size: 0.8rem; color: {theme::DEEP_NAVY}; font-weight: 700; z-index: 2;",
                                            "{year}"
                                        }
                                    }
                                }
                            }

                            div {
                                style: "position: absolute; left: 0; right: 0; bottom: {axis_height}px; height: 2px; background: {theme::DEEP_NAVY}; z-index: 2;",
                            }

                            for (i, exp) in experiences.iter().enumerate() {
                                {
                                    let (sy, sm) = parse_date(&exp.start_date);
                                    let (ey, em) = parse_date(&exp.end_date);
                                    let start = date_to_months(sy, sm);
                                    let end = date_to_months(ey, em);
                                    let x_start = (start - min_months) / total_months * 100.0;
                                    let x_end = (end - min_months) / total_months * 100.0;
                                    let width = (x_end - x_start).max(1.5);
                                    let row = row_assignments[i];
                                    let top = row * row_height + 8;
                                    let color = bar_color(i, &exp.kind);
                                    let is_selected = *selected.read() == Some(i);
                                    let outline = if is_selected { format!("3px solid {}", theme::MUTED_TEAL) } else { "none".to_string() };
                                    let z_index = if is_selected { 20 } else { 3 };
                                    let scale = if is_selected { "scale(1.03)" } else { "scale(1)" };
                                    let bar_center_pct = x_start + width / 2.0;

                                    rsx! {
                                        div {
                                            "data-bar-index": "{i}",
                                            "data-bar-center": "{bar_center_pct}",
                                            "data-bar-left": "{x_start}",
                                            "data-bar-right": "{x_start + width}",
                                            class: "timeline-bar",
                                            onclick: move |_| {
                                                if *selected.read() == Some(i) {
                                                    selected.set(None);
                                                } else {
                                                    selected.set(Some(i));
                                                }
                                            },
                                            style: "position: absolute; left: {x_start}%; width: {width}%; top: {top}px; height: {bar_height}px; background: {color}; border-radius: 3px; cursor: pointer; z-index: {z_index}; transition: all 0.15s ease; outline: {outline}; outline-offset: 2px; transform: {scale}; display: flex; align-items: center; padding: 0 0.6rem; overflow: hidden; white-space: nowrap;",

                                            span {
                                                style: "font-family: {theme::FONT_MONO}; font-size: 0.65rem; color: {theme::MINT_WHITE}; font-weight: 600; overflow: hidden; text-overflow: ellipsis;",
                                                "{exp.role} — {exp.company}"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    if let Some(idx) = *selected.read() {
                        {
                            let exp = &experiences[idx];
                            let start_display = format_date_display(&exp.start_date);
                            let end_display = format_date_display(&exp.end_date);
                            let color = bar_color(idx, &exp.kind);

                            rsx! {
                                div {
                                    style: "margin-top: 0.75rem; border: 3px solid {theme::DEEP_NAVY}; background: {theme::MINT_WHITE}; position: relative; flex-shrink: 0; max-height: 35vh; overflow-y: auto; display: flex;",

                                    div {
                                        style: "width: 6px; flex-shrink: 0; background: {color};",
                                    }

                                    div { style: "padding: 1.25rem 1.5rem; flex: 1; min-width: 0;",
                                        div { style: "display: flex; justify-content: space-between; align-items: flex-start; flex-wrap: wrap; gap: 0.5rem;",
                                            div {
                                                h3 {
                                                    style: "font-size: 1.4rem; font-weight: 700; color: {theme::DEEP_NAVY}; margin: 0; line-height: 1.2;",
                                                    "{exp.role}"
                                                }
                                                p {
                                                    style: "font-size: 1rem; color: {theme::DEEP_NAVY}; font-weight: 700; margin-top: 0.2rem;",
                                                    "{exp.company}"
                                                }
                                            }
                                            p {
                                                style: "font-family: {theme::FONT_MONO}; font-size: 0.8rem; color: {theme::DARK_BROWN}; white-space: nowrap;",
                                                "{start_display} — {end_display}"
                                            }
                                        }

                                        p {
                                            style: "margin-top: 0.75rem; color: {theme::DEEP_NAVY}; line-height: 1.6; font-size: 0.9rem;",
                                            "{exp.details}"
                                        }

                                        div {
                                            style: "display: flex; flex-wrap: wrap; gap: 0.4rem; margin-top: 0.75rem;",
                                            for tech in exp.tech.iter() {
                                                span {
                                                    style: "font-family: {theme::FONT_MONO}; font-size: 0.65rem; border: 2px solid {theme::DEEP_NAVY}; padding: 0.1rem 0.4rem; color: {theme::DEEP_NAVY};",
                                                    "{tech}"
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            div {
                class: "timeline-mobile",
                style: "background-color: {theme::DEEP_NAVY}; padding: 4rem 1rem 2rem 1rem;",

                h2 {
                    style: "font-size: 2rem; font-weight: 700; color: {theme::DARK_BROWN}; text-transform: uppercase; margin-bottom: 1.5rem;",
                    "EXPERIENCE & EDUCATION"
                }

                for &idx in sorted_indices.iter() {
                    {
                        let exp = &experiences[idx];
                        let start_display = format_date_display(&exp.start_date);
                        let end_display = format_date_display(&exp.end_date);
                        let color = bar_color(idx, &exp.kind);
                        let is_selected = *selected.read() == Some(idx);

                        rsx! {
                            div {
                                onclick: move |_| {
                                    if *selected.read() == Some(idx) {
                                        selected.set(None);
                                    } else {
                                        selected.set(Some(idx));
                                    }
                                },
                                style: "margin-bottom: 1rem; border-left: 5px solid {color}; background: {theme::MINT_WHITE}; cursor: pointer; transition: all 0.15s ease;",

                                div {
                                    style: "padding: 0.8rem 1rem;",
                                    div {
                                        style: "display: flex; justify-content: space-between; align-items: flex-start; gap: 0.5rem;",
                                        h3 {
                                            style: "font-size: 0.95rem; font-weight: 700; color: {theme::DEEP_NAVY}; margin: 0; line-height: 1.3;",
                                            "{exp.role}"
                                        }
                                        p {
                                            style: "font-family: {theme::FONT_MONO}; font-size: 0.65rem; color: {theme::DARK_BROWN}; white-space: nowrap; flex-shrink: 0;",
                                            "{start_display} — {end_display}"
                                        }
                                    }
                                    p {
                                        style: "font-size: 0.85rem; color: {theme::DEEP_NAVY}; font-weight: 600; margin-top: 0.15rem;",
                                        "{exp.company}"
                                    }
                                }

                                if is_selected {
                                    div {
                                        style: "padding: 0 1rem 0.8rem 1rem; border-top: 1px solid rgba(2, 24, 43, 0.1);",

                                        p {
                                            style: "margin-top: 0.6rem; color: {theme::DEEP_NAVY}; line-height: 1.6; font-size: 0.8rem;",
                                            "{exp.details}"
                                        }

                                        div {
                                            style: "display: flex; flex-wrap: wrap; gap: 0.3rem; margin-top: 0.6rem;",
                                            for tech in exp.tech.iter() {
                                                span {
                                                    style: "font-family: {theme::FONT_MONO}; font-size: 0.6rem; border: 1.5px solid {theme::DEEP_NAVY}; padding: 0.1rem 0.3rem; color: {theme::DEEP_NAVY};",
                                                    "{tech}"
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
