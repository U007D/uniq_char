use std::{collections::HashSet, ops::Not};

#[cfg(test)]
mod unit_tests;

pub fn uniq_char<S>(s: S) -> bool
where
    S: AsRef<str>,
{
    let s = s.as_ref();

    s.chars()
        .filter(|c| c.is_ascii() && c.is_ascii_whitespace().not())
        .try_fold(HashSet::new(), |mut seen, ch| match seen.insert(ch) {
            true => Some(seen),
            false => None,
        })
        .is_some()
}
