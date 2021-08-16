use colorful;

#[macro_export]
macro_rules! colors {
    ($color:ident; $($str:ident),*) => {
      match $color {
        $(stringify!($str) => colorful::Color::$str)*,
        _ => colorful::Color::White
       }
    }
}

pub fn color_from_str(color: &'static str) -> colorful::Color {
    match color {
        "Purple3" => colorful::Color::Purple3,
        "Purple4a" => colorful::Color::Purple4a,
        "Purple4b" => colorful::Color::Purple4b,
        "MediumPurple" => colorful::Color::MediumPurple,
        "MediumPurple1" => colorful::Color::MediumPurple1,
        "MediumPurple2a" => colorful::Color::MediumPurple2a,
        "MediumPurple2b" => colorful::Color::MediumPurple2b,
        "MediumPurple3a" => colorful::Color::MediumPurple3a,
        "MediumPurple3b" => colorful::Color::MediumPurple3b,
        "MediumPurple4" => colorful::Color::MediumPurple4,
        "Red" => colorful::Color::Red,
        "Red1" => colorful::Color::Red1,
        "Red3a" => colorful::Color::Red3a,
        "Red3b" => colorful::Color::Red3b,
        "Cyan" => colorful::Color::Cyan,
        "Cyan1" => colorful::Color::Cyan1,
        "Cyan2" => colorful::Color::Cyan2,
        "Cyan3" => colorful::Color::Cyan3,
        "Blue" => colorful::Color::Blue,
        "Blue1" => colorful::Color::Blue1,
        "Blue3a" => colorful::Color::Blue3a,
        "Blue3b" => colorful::Color::Blue3b,
        "DarkBlue" => colorful::Color::DarkBlue,
        "NavyBlue" => colorful::Color::NavyBlue,
        "RosyBrown" => colorful::Color::RosyBrown,
        "Yellow" => colorful::Color::Yellow,
        "Yellow1" => colorful::Color::Yellow1,
        "Yellow2" => colorful::Color::Yellow2,
        "Yellow3a" => colorful::Color::Yellow3a,
        "Yellow3b" => colorful::Color::Yellow3b,
        "Yellow4a" => colorful::Color::Yellow4a,
        "Yellow4b" => colorful::Color::Yellow4b,
        "GreenYellow" => colorful::Color::GreenYellow,
        "LightYellow" => colorful::Color::LightYellow,
        "LightYellow3" => colorful::Color::LightYellow3,
        "Orange1" => colorful::Color::Orange1,
        "Orange3" => colorful::Color::Orange3,
        "DarkOrange3a" => colorful::Color::DarkOrange3a,
        "DarkOrange3b" => colorful::Color::DarkOrange3b,
        "OrangeRed1" => colorful::Color::OrangeRed1,
        "Orange4a" => colorful::Color::Orange4a,
        "Orange4b" => colorful::Color::Orange4b,

        // if the color passed wasnt any of the above just use white.
        _ => colorful::Color::White,
    }
}
