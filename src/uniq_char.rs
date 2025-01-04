use std::collections::HashSet;

#[cfg(test)]
mod unit_tests;

pub fn uniq_char<S>(s: S) -> bool
where
    S: AsRef<str>,
{
    let s = s.as_ref();

    s.chars()
        .filter(char::is_ascii)
        .try_fold(HashSet::new(), |mut res, ch| match res.insert(ch) {
            true => Some(res),
            false => None,
        })
        .is_some()
}
