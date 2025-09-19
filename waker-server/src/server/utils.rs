use std::env::var;

pub fn is_env<S>(name: S) -> bool
where
    S: AsRef<str>,
{
    !var(name.as_ref()).unwrap_or_default().is_empty()
}
