use std::collections::HashMap;
use std::fmt;
use std::ops;

#[derive(Debug, Clone)]
pub struct FieldError;

impl fmt::Display for FieldError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid field formulation! Must be: ...")
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Field {
    name: String,
    default: String,
    help: String,
    position: i8,
    options: Vec<String>,
    width: i8,
}

impl Field {
    pub fn new() -> Field {
        Field::default()
    }
    pub fn default() -> Field {
        Field {
            name: String::new(),
            default: String::new(),
            help: String::new(),
            position: 0,
            options: Vec::new(),
            width: 10,
        }
    }
    pub fn build(
        name: String,
        default: String,
        help: String,
        position: i8,
        options: Vec<String>,
        width: i8,
    ) -> Result<Field, FieldError> {
        Ok(Field {
            name: name,
            default: default,
            help: help,
            position: position,
            options: options,
            width: width,
        })
    }

    pub fn clear(&mut self) -> () {
        self.update(Field::default());
    }

    pub fn update(&mut self, _rhs: Field) {
        self.name = _rhs.name;
        self.default = _rhs.default;
        self.help = _rhs.help;
        self.position = _rhs.position;
        self.options = _rhs.options;
        self.width = _rhs.width;
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Card {
    values: HashMap<String, Field>,
}

pub struct CardError;

impl fmt::Display for CardError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid card formulation! Must be: ...")
    }
}

impl Card {
    pub fn new(fields: Vec<Field>) -> Card {
        let values: HashMap<String, Field> =
            fields.into_iter().map(|f| (f.name.clone(), f)).collect();

        Card { values: values }
    }

    pub fn default() -> Card {
        Card {
            values: HashMap::new(),
        }
    }

    pub fn clear(&mut self) -> () {
        self.values.clear();
    }
}

impl ops::Add<Card> for Card {
    type Output = Card;
    fn add(self, _rhs: Card) -> Card {
        Card {
            values: self.values.into_iter().chain(_rhs.values).collect(),
        }
    }
}

impl ops::Add<Field> for Card {
    type Output = Card;
    fn add(mut self, _rhs: Field) -> Card {
        self.values.insert(_rhs.name.clone(), _rhs);
        Card {
            values: self.values,
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct KeyWord {
    keyword: String,
    cards: Vec<Card>,
    is_commented: bool,
}

impl KeyWord {
    pub fn new(keyword: String, cards: Vec<Card>, is_commented: bool) -> KeyWord {
        KeyWord {
            keyword: keyword,
            cards: cards,
            is_commented: is_commented,
        }
    }

    pub fn empty_keyword() -> KeyWord {
        KeyWord {
            keyword: String::new(),
            cards: Vec::new(),
            is_commented: false,
        }
    }

    pub fn clear(&mut self) -> () {
        self.cards.clear();
        self.keyword = String::new();
        self.is_commented = true;
    }

    pub fn comment(&mut self) -> () {
        self.is_commented = true;
    }

    pub fn uncomment(&mut self) -> () {
        self.is_commented = false;
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Deck {
    keywords: Vec<KeyWord>,
    prefix: String,
}

impl Deck {
    pub fn new(keywords: Vec<KeyWord>) -> Deck {
        Deck::new_with_prefix(keywords, String::new())
    }

    pub fn new_with_prefix(keywords: Vec<KeyWord>, prefix: String) -> Deck {
        Deck {
            keywords: keywords,
            prefix: prefix,
        }
    }

    pub fn create_blank_deck() -> Deck {
        Deck {
            keywords: Vec::new(),
            prefix: String::new(),
        }
    }

    pub fn clear_keywords(&mut self) -> () {
        self.keywords.clear();
    }

    pub fn empty(&mut self) -> () {
        self.clear_keywords();
        self.prefix.clear();
    }
}

impl ops::Add<Deck> for Deck {
    type Output = Deck;

    fn add(mut self, _rhs: Deck) -> Deck {
        self.keywords.extend(_rhs.keywords);
        Deck {
            keywords: self.keywords,
            prefix: self.prefix,
        }
    }
}

impl ops::Add<KeyWord> for Deck {
    type Output = Deck;

    fn add(mut self, _rhs: KeyWord) -> Deck {
        self.keywords.push(_rhs);
        Deck {
            keywords: self.keywords,
            prefix: self.prefix,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_field() {
        let vec_str: Vec<String> = Vec::new();
        let f = Field::default();

        // Initial implementation has new method returning
        // the Field::default() so we need to make sure they
        // return the same thing so our tests will catch
        // anything if the api changes in the future between
        // new and default
        let f2 = Field::new();

        assert_eq!(String::new(), f.name);
        assert_eq!(String::new(), f.default);
        assert_eq!(String::new(), f.help);
        assert_eq!(0, f.position);
        assert_eq!(vec_str, f.options);
        assert_eq!(10, f.width);

        assert_eq!(String::new(), f2.name);
        assert_eq!(String::new(), f2.default);
        assert_eq!(String::new(), f2.help);
        assert_eq!(0, f2.position);
        assert_eq!(vec_str, f2.options);
        assert_eq!(10, f2.width);

        assert_eq!(f2.name, f.name);
        assert_eq!(f2.default, f.default);
        assert_eq!(f2.help, f.help);
        assert_eq!(f2.position, f.position);
        assert_eq!(f2.options, f.options);
        assert_eq!(f2.width, f.width);
    }
    #[test]
    fn test_build_field() {
        let f_name = String::from("test");
        let f_default = String::from("0.0");
        let f_help = String::from("This is a test");
        let f_position = 0;
        let f_options: Vec<String> = Vec::new();
        let f_width = 10;

        let f = match Field::build(
            f_name.clone(),
            f_default.clone(),
            f_help.clone(),
            f_position.clone(),
            f_options.clone(),
            f_width.clone(),
        ) {
            Ok(f) => f,
            Err(e) => panic!("Unable to build field {e}!! "),
        };

        assert_eq!(f_name, f.name);
        assert_eq!(f_default, f.default);
        assert_eq!(f_help, f.help);
        assert_eq!(f_position, f.position);
        assert_eq!(f_options, f.options);
        assert_eq!(f_width, f.width);
    }
    #[test]
    fn test_clear_field() {
        let f_name = String::from("test");
        let f_default = String::from("0.0");
        let f_help = String::from("This is a test");
        let f_position = 0;
        let f_options: Vec<String> = Vec::new();
        let f_width = 10;

        let mut f = match Field::build(f_name, f_default, f_help, f_position, f_options, f_width) {
            Ok(f) => f,
            Err(e) => panic!("Unable to build Field ({e})!!"),
        };

        let f2 = Field::default();

        f.clear();

        assert_eq!(f, f2);
    }
    #[test]
    fn test_update_field() {
        // default may change, but for the purposes
        // of this test, the initialized values are
        // irrelivant
        let mut f1 = Field::default();

        let f2 = match Field::build(
            String::from("Card2"),
            String::from("1.0"),
            String::from("This is a test"),
            20,
            vec![],
            10,
        ) {
            Ok(f) => f,
            Err(e) => panic!("Unable to build field ({e})!!"),
        };

        f1.update(f2.clone());

        assert_eq!(f1, f2);
    }
    #[test]
    fn test_default_card() {
        let t_map: HashMap<String, Field> = HashMap::new();

        let c = Card::default();

        assert_eq!(c.values, t_map);
    }
    #[test]
    fn test_new_card() {
        let mut t_vec: Vec<Field> = Vec::new();

        t_vec.push(Field::new());

        let custom_opts = vec![String::from("option1"), String::from("option2")];

        let custom_field = match Field::build(
            String::from("Custom"),
            String::from("default"),
            String::from("This is the help"),
            30,
            custom_opts,
            10,
        ) {
            Ok(f) => f,
            Err(e) => panic!("Unable to build field ({e})!!"),
        };

        t_vec.push(custom_field.clone());

        let c = Card::new(t_vec.clone());

        assert_eq!(c.values["Custom"], t_vec[1]);
        assert_eq!(c.values[""], t_vec[0]);
    }

    #[test]
    fn test_clear_card() {
        let fields = vec![
            Field {
                name: String::from("Field1"),
                default: String::from("1.0"),
                help: String::from("Help Field 1"),
                position: 0,
                options: vec![],
                width: 10,
            },
            Field {
                name: String::from("Field2"),
                default: String::from("2.0"),
                help: String::from("Help Field2"),
                position: 10,
                options: vec![String::from("option1"), String::from("option2")],
                width: 10,
            },
        ];

        let c = Card::new(fields.clone());

        assert_eq!(c.values["Field1"], fields[0]);
        assert_eq!(c.values["Field2"], fields[1]);
    }
    #[test]
    fn test_add_card() {}
    #[test]
    fn test_new_keyword() {}
    #[test]
    fn test_empty_keyword() {}
    #[test]
    fn test_add_keyword() {}
    #[test]
    fn test_comment_keyword() {}
    #[test]
    fn test_uncomment_keyword() {}
    #[test]
    fn test_new_deck() {}
    #[test]
    fn test_new_deck_with_prefix() {}
    #[test]
    fn test_new_blank_deck() {}
    #[test]
    fn test_clear_keywords() {}
    #[test]
    fn test_empty_deck() {}
    #[test]
    fn test_add_keyword_to_deck() {}
    #[test]
    fn test_add_deck_to_deck() {}
}
