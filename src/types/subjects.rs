use super::Subjects;

impl ToString for Subjects {
    fn to_string(&self) -> String {
        match self {
            Subjects::Math => String::from("mh"),
            Subjects::Geometry => String::from("gm"),
            Subjects::RussianLang => String::from("ru"),
            Subjects::UzbekLang => String::from("uz"),
            Subjects::EnglishLang => String::from("en"),
            Subjects::WorldHistory => String::from("wh"),
            Subjects::HistoryOfUzbekistan => String::from("uh"),
            Subjects::Biology => String::from("bo"),
            Subjects::Chemistry => String::from("ch"),
            Subjects::Drowing => String::from("dr"),
            Subjects::Physics => String::from("ph"),
            Subjects::Literature => String::from("li"),
            Subjects::Economy => String::from("ec"),
        }
    }
}

impl From<&str> for Subjects {
    fn from(value: &str) -> Self {
        match value {
            "mh" => Subjects::Math,
            "gm" => Subjects::Geometry,
            "ru" => Subjects::RussianLang,
            "uz" => Subjects::UzbekLang,
            "en" => Subjects::EnglishLang,
            "wh" => Subjects::WorldHistory,
            "uh" => Subjects::HistoryOfUzbekistan,
            "bo" => Subjects::Biology,
            "ch" => Subjects::Chemistry,
            "dr" => Subjects::Drowing,
            "ph" => Subjects::Physics,
            "li" => Subjects::Literature,
            "ec" => Subjects::Economy,
            _ => panic!("Invalid string"),
        }
    }
}
