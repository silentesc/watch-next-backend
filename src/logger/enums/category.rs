use std::fmt;

pub enum Category {
    Setup,
    Register,
    Login,
}

impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let category_str = match self {
            Category::Setup => String::from("setup"),
            Category::Register => String::from("register"),
            Category::Login => String::from("login"),
        };
        write!(f, "{}", category_str)
    }
}
