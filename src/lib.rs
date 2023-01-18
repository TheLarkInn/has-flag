pub fn has_flag(flag: &str) -> bool {
    _has_flag(std::env::args(), flag)
}

fn _has_flag<I: Iterator<Item = String>>(mut args: I, flag: &str) -> bool {
    let prefix = if flag.starts_with('-') {
        ""
    } else {
        if flag.len() == 1 {
            "-"
        } else {
            "--"
        }
    };

    let position = args.position(|arg| arg == format!("{}{}", prefix, flag));
    let terminator_position = args.position(|arg| arg == "--");
    position.is_some() && (!terminator_position.is_some() || position < terminator_position)
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     #[test]
//     fn all_args() {
//     }
// }
