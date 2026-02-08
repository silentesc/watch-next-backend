use std::fmt;

pub enum Category {
    Setup,
}

impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let category_str = match self {
            Category::Setup => String::from("setup"),
        };
        write!(f, "{}", category_str)
    }
}
