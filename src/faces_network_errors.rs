use std::fmt::Display;

#[derive(Debug)]
pub enum FNtwrkCommonTypesErrors {
    ParsingFailure(String),
}

impl Display for FNtwrkCommonTypesErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FNtwrkCommonTypesErrors::ParsingFailure(error) => {
                write!(f, "FNtwrkCommonTypesErrors : ParsingFailure [{:?}]", error)
            }
        }
    }
}

impl std::error::Error for FNtwrkCommonTypesErrors {}
