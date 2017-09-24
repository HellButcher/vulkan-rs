use std::ascii::AsciiExt;

pub enum CaseStyle {
    UnspecifiedCase,
    LowerCase,
    UpperCase,
    CamelCase,
    PascalCase,
    SnakeCase,
    ScreamingSnakeCase,
}

impl CaseStyle {
    #[inline]
    pub fn apply_to_snake(&self, snake: &str) -> String {
        use self::CaseStyle::*;
        match *self {
            UnspecifiedCase => snake.to_owned(),
            LowerCase => snake.to_ascii_lowercase(),
            UpperCase => snake.to_ascii_uppercase(),
            CamelCase => {
                let mut camel = String::new();
                let mut capitalize = false;
                for ch in snake.chars() {
                    if ch == '_' {
                        capitalize = true;
                    } else if capitalize {
                        camel.push(ch.to_ascii_uppercase());
                        capitalize = false;
                    } else {
                        camel.push(ch.to_ascii_lowercase());
                    }
                }
                camel
            },
            PascalCase => {
                let camel = CamelCase.apply_to_snake(snake);
                camel[..1].to_ascii_uppercase() + &camel[1..]
            },
            SnakeCase => snake.to_ascii_lowercase(),
            ScreamingSnakeCase => snake.to_ascii_uppercase(),
        }
    }

    #[inline]
    pub fn apply_to_camel(&self, camel: &str) -> String {
        use self::CaseStyle::*;
        match *self {
            UnspecifiedCase => camel.to_owned(),
            LowerCase  => camel.to_ascii_lowercase(),
            UpperCase  => camel.to_ascii_uppercase(),
            CamelCase  => camel[..1].to_ascii_lowercase() + &camel[1..],
            PascalCase => camel[..1].to_ascii_uppercase() + &camel[1..],
            SnakeCase  => {
                let mut snake = String::new();
                let mut multi = 0;
                for ch in camel.chars() {
                    if ch.is_lowercase() {
                        if multi > 1 {
                            snake.push('_');
                        }
                        if multi > 0 {
                            multi = 0;
                        }
                        multi -= 1;
                        snake.push(ch);
                    } else if ch.is_uppercase() {
                        if multi < 0 {
                            multi = 0;
                            snake.push('_');
                        }
                        multi += 1;
                        snake.push(ch.to_ascii_lowercase());
                    } else {
                        snake.push(ch);
                    }
                }
                snake
            },
            ScreamingSnakeCase => SnakeCase.apply_to_camel(camel).to_ascii_uppercase(),
        }
    }
}

impl ::std::str::FromStr for CaseStyle {
    type Err = ();

    #[inline]
    fn from_str(rename_all_str: &str) -> Result<Self, Self::Err> {
        use self::CaseStyle::*;
        match rename_all_str {
            "lowercase" => Ok(LowerCase),
            "UPPERCASE" => Ok(UpperCase),
            "camelCase" => Ok(CamelCase),
            "PascalCase" => Ok(PascalCase),
            "snake_case" => Ok(SnakeCase),
            "SCREAMING_SNAKE_CASE" => Ok(ScreamingSnakeCase),
            ""| "UnspecifiedCase" => Ok(UnspecifiedCase),
            _ => Err(()),
        }
    }
}
 impl ::std::fmt::Display for CaseStyle {
     #[inline]
     fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
         use self::CaseStyle::*;
         match *self {
             UnspecifiedCase => write!(fmt, "UnspecifiedCase"),
             LowerCase => write!(fmt, "lowercase"),
             UpperCase => write!(fmt, "UPPERCASE"),
             CamelCase => write!(fmt, "camelCase"),
             PascalCase => write!(fmt, "PascalCase"),
             SnakeCase => write!(fmt, "snake_case"),
             ScreamingSnakeCase => write!(fmt, "SCREAMING_SNAKE_CASE"),
         }
     }
 }


 #[test]
 fn apply_to_camels() {
     use self::CaseStyle::*;
     for &(original, lower, upper, camel, pascal, snake, screaming) in
         &[
             ("lower", "lower", "LOWER", "lower", "Lower", "lower", "LOWER"),
             ("Capital", "capital", "CAPITAL", "capital", "Capital", "capital", "CAPITAL"),
             ("camelCase", "camelcase", "CAMELCASE", "camelCase", "CamelCase", "camel_case", "CAMEL_CASE"),
             ("PascalCase", "pascalcase", "PASCALCASE", "pascalCase", "PascalCase", "pascal_case", "PASCAL_CASE"),
             ("a", "a", "A", "a", "A", "a", "A"),
             ("B", "b", "B", "b", "B", "b", "B"),
             ("aB", "ab", "AB", "aB", "AB", "a_b", "A_B"),
             ("Z12", "z12", "Z12", "z12", "Z12", "z12", "Z12"),
         ] {
         assert_eq!(None.apply_to_camel(original), original);
         assert_eq!(LowerCase.apply_to_camel(original), lower);
         assert_eq!(UpperCase.apply_to_camel(original), upper);
         assert_eq!(CamelCase.apply_to_camel(original), camel);
         assert_eq!(PascalCase.apply_to_camel(original), pascal);
         assert_eq!(SnakeCase.apply_to_camel(original), snake);
         assert_eq!(ScreamingSnakeCase.apply_to_camel(original), screaming);
     }
 }

 #[test]
 fn apply_to_snakes() {
     use self::CaseStyle::*;
     for &(original, lower, upper, camel, pascal, snake, screaming) in
         &[
             ("lower", "lower", "LOWER", "lower", "Lower", "lower", "LOWER"),
             ("Capital", "capital", "CAPITAL", "capital", "Capital", "capital", "CAPITAL"),
             ("snake_case", "snake_case", "SNAKE_CASE", "snakeCase", "SnakeCase", "snake_case", "SNAKE_CASE"),
             ("MixEd_caSe", "mixed_case", "MIXED_CASE", "mixedCase", "MixedCase", "mixed_case", "MIXED_CASE"),
             ("a", "a", "A", "a", "A", "a", "A"),
             ("B", "b", "B", "b", "B", "b", "B"),
             ("a_B", "a_b", "A_B", "aB", "AB", "a_b", "A_B"),
             ("Z12", "z12", "Z12", "z12", "Z12", "z12", "Z12"),
             ("Z_12", "z_12", "Z_12", "z12", "Z12", "z_12", "Z_12"),
         ] {
         assert_eq!(None.apply_to_snake(original), original);
         assert_eq!(LowerCase.apply_to_snake(original), lower);
         assert_eq!(UpperCase.apply_to_snake(original), upper);
         assert_eq!(CamelCase.apply_to_snake(original), camel);
         assert_eq!(PascalCase.apply_to_snake(original), pascal);
         assert_eq!(SnakeCase.apply_to_snake(original), snake);
         assert_eq!(ScreamingSnakeCase.apply_to_snake(original), screaming);
     }
 }
