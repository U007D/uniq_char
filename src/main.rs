#[expect(clippy::wildcard_imports, reason = "Ok to use wildcard import on `shared_consts`.")]
use lib::{self, shared_consts::*, uniq_char, Result};

#[expect(
    clippy::missing_const_for_fn,
    reason = "Remove this attribute when function body is implemented"
)]
#[termination::display]
fn main() -> Result<()> {
    let arg = std::env::args_os()
        .nth(CMD_LINE_ARG_POSITION)
        .map_or_else(String::new, |oss| oss.to_string_lossy().to_string());

    let res = uniq_char(&arg);

    let clause = match res {
        true => "only unique characters",
        false => "duplicate characters",
    };
    println!("The argument {arg} contains {clause}.");

    Ok(())
}
