pub fn has_flag(flag: &str) -> bool {
    _has_flag(std::env::args(), flag)
}

fn _has_flag<I: Iterator<Item = String>>(args: I, flag: &str) -> bool {
    let prefix = if flag.starts_with('-') {
        ""
    } else {
        if flag.len() == 1 {
            "-"
        } else {
            "--"
        }
    };

    let formatted_flag = format!("{}{}", prefix, flag);
    args.take_while(|arg| arg != "--").any(|arg| arg == formatted_flag)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn args_with_value_not_matching_double_dash() {
        let args = vec!["--foo", "--unicorn=rainbow", "--bar"];
        let expected_value = "unicorn=rainbow";

        assert!(_has_flag(
            args.into_iter().map(ToString::to_string),
            expected_value
        ))
    }

    #[test]
    fn args_matching_value_before_terminator() {
        let args = vec!["--unicorn", "--", "--foo"];
        let expected_value = "unicorn";

        assert!(_has_flag(
            args.into_iter().map(ToString::to_string),
            expected_value
        ))
    }

    #[test]
    fn args_not_matching_value_after_terminator() {
        let args = vec!["--foo", "--", "--unicorn"];
        let expected_value = "unicorn";

        assert!(!_has_flag(
            args.into_iter().map(ToString::to_string),
            expected_value
        ))
    }

    #[test]
    fn args_not_matching_value_not_in_args() {
        let args = vec!["--foo"];
        let expected_value = "unicorn";

        assert!(!_has_flag(
            args.into_iter().map(ToString::to_string),
            expected_value
        ))
    }

    #[test]
    fn args_with_sequential_single_dash_input() {
        let args = vec!["-f", "-u", "-b"];
        let expected_value = "-u";

        assert!(_has_flag(
            args.into_iter().map(ToString::to_string),
            expected_value
        ))
    }

    #[test]
    fn args_with_sequential_single_dash_and_match_before_terminator() {
        let args = vec!["-u", "--", "-f"];
        let expected_value = "-u";

        assert!(_has_flag(
            args.into_iter().map(ToString::to_string),
            expected_value
        ))
    }

    #[test]
    fn args_with_sequential_single_dash_and_no_match_after_terminator() {
        let args = vec!["-u", "--", "-f"];
        let expected_value = "-f";

        assert!(!_has_flag(
            args.into_iter().map(ToString::to_string),
            expected_value
        ))
    }

    #[test]
    fn args_without_double_dash_input() {
        assert!(_has_flag(
            vec!["--unicorn", "--foo", "-f"]
                .into_iter()
                .map(ToString::to_string),
            "unicorn"
        ));
    }

    #[test]
    fn args_with_double_dash_input() {
        assert!(_has_flag(
            vec!["--unicorn", "--foo", "-f"]
                .into_iter()
                .map(ToString::to_string),
            "--unicorn"
        ));
    }

    #[test]
    fn args_with_single_dash_input() {
        assert!(_has_flag(
            vec!["--unicorn", "--foo", "-f"]
                .into_iter()
                .map(ToString::to_string),
            "-f"
        ));
    }

    #[test]
    fn args_without_single_dash_input() {
        assert!(_has_flag(
            vec!["--unicorn", "--foo", "-f"]
                .into_iter()
                .map(ToString::to_string),
            "f"
        ));
    }

    #[test]
    fn args_that_doesnt_exist() {
        assert!(!_has_flag(
            vec!["--unicorn", "--foo", "-f"]
                .into_iter()
                .map(ToString::to_string),
            "rainbow"
        ));
    }
}
