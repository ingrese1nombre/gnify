#[macro_export]
macro_rules! text {
    ($name: ident $(=> $(pattern: $rgx: literal;)? $(min: $min: expr;)? $(max: $max: expr;)?)?) => {
        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, ::serde::Serialize, ::serde::Deserialize, std::hash::Hash)]
        pub struct $name (String);
        
        impl $name {
            pub fn value(&self) -> &str {
                &self.0
            }
        }

        // impl ::serde::Serialize for $name {
        //     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        //     where
        //         S: serde::Serializer,
        //     {
        //         self.0.serialize(serializer)
        //     }
        // }

        // impl<'de> ::serde::Deserialize<'de> for $name {
        //     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        //     where
        //         D: ::serde::Deserializer<'de> 
        //     {
        //             let value = <String as ::serde::Deserialize>::deserialize(deserializer)?;
        //             Ok(Self(value))
        //     }
        // }

        impl ::std::ops::Deref for $name {
            type Target = String;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl std::str::FromStr for $name {
            type Err = $crate::error::InvalidValue;
        
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                use $crate::error::InvalidValue;

                $(
                    $(
                        use ::regex::Regex;
                        use ::once_cell::sync::Lazy;
                        
                        static RE: Lazy<Regex> = Lazy::new(|| {Regex::new($rgx).expect("invalid regex")});
                        if !RE.is_match(s) {
                            return Err(InvalidValue::new(stringify!($name)));
                        }
                    )?
                    $(
                        if s.len() < $min {
                            return Err(InvalidValue::new(stringify!($name)));
                        }
                    )?
                    $(
                        if s.len() > $max {
                            return Err(InvalidValue::new(stringify!($name)));
                        }
                    )?
                )?
                Ok($name(s.to_string()))
            }
        }
    };
    ($name: ident : $rgx: literal) => {
        $crate::text!{$name => pattern: $rgx;}
    };
}