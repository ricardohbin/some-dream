pub struct Orange {}
pub struct Red {}
pub struct Blue {}
pub struct Purple {}
pub struct Green {}
pub struct Gray {}
pub struct Yellow {}

pub trait Color {
    fn pattern(&self) -> &'static str;
}

impl Color for Orange {
    fn pattern(&self) -> &'static str {
        "\u{001b}[38;5;166m_\u{001b}[0m"
    }
}

impl Color for Red {
    fn pattern(&self) -> &'static str {
        "\u{001b}[38;5;196m_\u{001b}[0m"
    }
}

impl Color for Blue {
    fn pattern(&self) -> &'static str {
        "\u{001b}[38;5;33m_\u{001b}[0m"
    }
}

impl Color for Purple {
    fn pattern(&self) -> &'static str {
        "\u{001b}[38;5;99m_\u{001b}[0m"
    }
}

impl Color for Green {
    fn pattern(&self) -> &'static str {
        "\u{001b}[38;5;82m_\u{001b}[0m"
    }
}

impl Color for Gray {
    fn pattern(&self) -> &'static str {
        "\u{001b}[38;5;242m_\u{001b}[0m"
    }
}

impl Color for Yellow {
    fn pattern(&self) -> &'static str {
        "\u{001b}[38;5;226m_\u{001b}[0m"
    }
}

pub fn paint(color: Box<dyn Color>, pattern: &str, text: &str) -> String {
    let mut color_out: String = String::from("");

    for char in text.chars() {
        let string_char = char.to_string();
        if string_char.as_str() == pattern {
            color_out.push_str(color.pattern().replace("_", pattern).as_str());
        } else {
            color_out.push_str(string_char.as_str());
        }
    }

    color_out
}