// TODO: Implement `TryFrom<String>` and `TryFrom<&str>` for the `TicketTitle` type,
//   enforcing that the title is not empty and is not longer than 50 bytes.
//   Implement the traits required to make the tests pass too.
use std::fmt::Display;
#[derive(Debug, PartialEq, Clone)]
pub struct TicketTitle(String);

#[derive(Debug)]
pub enum TicketTitleError {
    Empty,
    TooShort,
}

impl Display for TicketTitleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => write!(f, "The title cannot be empty"),
            Self::TooShort => write!(f, "The title cannot be longer than 50 bytes"),
        }
    }
}

impl TryFrom<&str> for TicketTitle {
    type Error = TicketTitleError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let v = String::try_from(value).unwrap();
        if v.is_empty() {
            return Err(TicketTitleError::Empty);
            //return Err("The title cannot be empty".to_string());
        }

        if v.len() > 50 {
            return Err(TicketTitleError::TooShort);
            //return Err("The title cannot be longer than 50 bytes".to_string());
        }

        Ok(Self(v))
    }
}

impl TryFrom<String> for TicketTitle {
    type Error = TicketTitleError;
    fn try_from(v: String) -> Result<Self, Self::Error> {
        if v.is_empty() {
            return Err(TicketTitleError::Empty);
            //return Err("The title cannot be empty".to_string());
        }

        if v.len() > 50 {
            return Err(TicketTitleError::TooShort);
            //return Err("The title cannot be longer than 50 bytes".to_string());
        }

        Ok(Self(v))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;

    #[test]
    fn test_try_from_string() {
        let title = TicketTitle::try_from("A title".to_string()).unwrap();
        assert_eq!(title.0, "A title");
    }

    #[test]
    fn test_try_from_empty_string() {
        let err = TicketTitle::try_from("".to_string()).unwrap_err();
        assert_eq!(err.to_string(), "The title cannot be empty");
    }

    #[test]
    fn test_try_from_long_string() {
        let title =
            "A title that's definitely longer than what should be allowed in a development ticket"
                .to_string();
        let err = TicketTitle::try_from(title).unwrap_err();
        assert_eq!(err.to_string(), "The title cannot be longer than 50 bytes");
    }

    #[test]
    fn test_try_from_str() {
        let title = TicketTitle::try_from("A title").unwrap();
        assert_eq!(title.0, "A title");
    }
}
