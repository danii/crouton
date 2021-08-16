use colorful;

macro_rules! colors {
  ($color:ident; $($str:ident),*) => {
    match $color {
      $(stringify!($str) => colorful::Color::$str),*,
      _ => colorful::Color::White
     }
  }
}

pub fn color_from_str(color: &'static str) -> colorful::Color {
    colors! {
        color;
        
        Purple3,
        Purple4a,
        Purple4b,
        MediumPurple,
        MediumPurple1,
        MediumPurple2a,
        MediumPurple2b,
        MediumPurple3a,
        MediumPurple3b,
        MediumPurple4,
        Red,
        Red1,
        Red3a,
        Red3b,
        Cyan,
        Cyan1,
        Cyan2,
        Cyan3,
        Blue,
        Blue1,
        Blue3a,
        Blue3b,
        DarkBlue,
        NavyBlue,
        RosyBrown,
        Yellow,
        Yellow1,
        Yellow2,
        Yellow3a,
        Yellow3b,
        Yellow4a,
        Yellow4b,
        GreenYellow,
        LightYellow,
        LightYellow3,
        Orange1,
        Orange3,
        DarkOrange3a,
        DarkOrange3b,
        OrangeRed1,
        Orange4a,
        Orange4b,
        White
    }
}
