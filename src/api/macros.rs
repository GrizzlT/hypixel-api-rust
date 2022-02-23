macro_rules! display_enum_with_case {
    ($e:ident, $case:ident) => {
        impl ::std::fmt::Display for $e {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::result::Result<(), ::std::fmt::Error> {
                write!(f, "{}", ::convert_case::Casing::<::std::string::String>::to_case(&format!("{:?}", self), ::convert_case::Case::$case))
            }
        }
    }
}