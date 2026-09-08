pub fn parse_date(date_str: &str) -> (i32, u32) {
    if date_str.is_empty() {
        return (2026, 3);
    }
    let parts: Vec<&str> = date_str.split('-').collect();
    let year = parts[0].parse::<i32>().unwrap_or(2026);
    let month = if parts.len() > 1 {
        parts[1].parse::<u32>().unwrap_or(1)
    } else {
        1
    };
    (year, month)
}

pub fn date_to_months(year: i32, month: u32) -> f64 {
    (year as f64) * 12.0 + (month as f64)
}

pub fn bar_color(index: usize, kind: &str) -> &'static str {
    if kind == "education" {
        const EDU_COLORS: &[&str] = &["#2D6A4F", "#1a5276", "#4A6741"];
        return EDU_COLORS[index % EDU_COLORS.len()];
    }
    const COLORS: &[&str] = &[
        "#02182B", "#D65108", "#2D6A4F", "#6B4C8A", "#B85C38", "#1a5276",
    ];
    COLORS[index % COLORS.len()]
}

pub fn format_date_display(date_str: &str) -> String {
    if date_str.is_empty() {
        return "Present".to_string();
    }
    let parts: Vec<&str> = date_str.split('-').collect();
    if parts.len() == 2 {
        let month_name = match parts[1] {
            "01" => "Jan",
            "02" => "Feb",
            "03" => "Mar",
            "04" => "Apr",
            "05" => "May",
            "06" => "Jun",
            "07" => "Jul",
            "08" => "Aug",
            "09" => "Sep",
            "10" => "Oct",
            "11" => "Nov",
            "12" => "Dec",
            _ => parts[1],
        };
        format!("{} {}", month_name, parts[0])
    } else {
        date_str.to_string()
    }
}
