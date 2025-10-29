#[macro_export]
macro_rules! value_t {
    ($matches:expr, $name:expr, $ty:ty) => {{
        match $matches.get_one::<$ty>($name) {
            Some(val) => Ok(*val),
            None => match $matches.get_one::<String>($name) {
                Some(val) => val.parse::<$ty>().map_err(|err| {
                    ::clap::Error::raw(
                        ::clap::error::ErrorKind::InvalidValue,
                        format!("Invalid value for '{}': {}", $name, err),
                    )
                }),
                None => Err(::clap::Error::raw(
                    ::clap::error::ErrorKind::InvalidValue,
                    format!("Argument '{}' not provided", $name),
                )),
            },
        }
    }};
}

#[macro_export]
macro_rules! values_t {
    ($matches:expr, $name:expr, $ty:ty) => {{
        let mut values = Vec::new();
        if let Some(existing) = $matches.get_many::<$ty>($name) {
            values.extend(existing.cloned());
            Ok(values)
        } else if let Some(existing) = $matches.get_many::<String>($name) {
            for val in existing {
                match val.parse::<$ty>() {
                    Ok(parsed) => values.push(parsed),
                    Err(err) => {
                        return Err(::clap::Error::raw(
                            ::clap::error::ErrorKind::InvalidValue,
                            format!("Invalid value for '{}': {}", $name, err),
                        ));
                    }
                }
            }
            Ok(values)
        } else {
            Err(::clap::Error::raw(
                ::clap::error::ErrorKind::InvalidValue,
                format!("Argument '{}' not provided", $name),
            ))
        }
    }};
}
