use uuid::{
    Uuid,
    ParseError
};
use std::{
    fmt::{
        Display,
        Formatter,
        Result as FormatResult
    }
};

#[derive(Deserialize, Serialize, Clone, Copy, Debug, PartialEq, Default, Hash, Eq)]
pub struct ThreadUuid(pub Uuid);

impl ThreadUuid {
    pub fn to_query_parameter(self) -> String {
        format!("{}={}", PARAM_NAME, self.0)
    }
    pub fn parse_str(input: &str) -> Result<Self, ParseError> {
        Uuid::parse_str(input).map(ThreadUuid)
    }
}

const PARAM_NAME: &str = "thread_uuid";

impl Display for ThreadUuid {
    fn fmt(&self, f: &mut Formatter) -> FormatResult {
        write!(f, "{}", self.0)
    }
}

impl From<Uuid> for ThreadUuid {
    fn from(uuid: Uuid) -> ThreadUuid {
        ThreadUuid(uuid)
    }
}

#[cfg(feature = "rocket_support")]
mod rocket {
    use super::*;
    use ::rocket::http::RawStr;
    use ::rocket::request::FromParam;
    use crate::uuid_from_param;
    use crate::uuid_from_form;
    use ::rocket::request::{FromForm, FormItems};

    impl<'a> FromParam<'a> for ThreadUuid {
        type Error = &'a RawStr;

        #[inline]
        fn from_param(param: &'a RawStr) -> Result<Self, Self::Error> {
            uuid_from_param(param).map(ThreadUuid)
        }
    }


    impl<'f> FromForm<'f> for ThreadUuid {
        type Error = ();

        #[inline]
        fn from_form(items: &mut FormItems<'f>, strict: bool) -> Result<Self, ()> {
            uuid_from_form(items, strict, PARAM_NAME)
                .map(ThreadUuid)
        }
    }
}
