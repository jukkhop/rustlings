// errors3.rs
// This is a program that is trying to use a completed version of the
// `total_cost` function from the previous exercise. It's not working though--
// we can't use the `?` operator in the `main()` function! Why not?
// What should we do instead? Scroll for hints!

use std::num::ParseIntError;

fn buy_items(mut tokens: i32, pretend_user_input: &str) -> String {

    match total_cost(pretend_user_input) {
        Ok(cost) => {
            if cost > tokens {
                return format!("You can't afford that many!");
            } else {
                tokens -= cost;
                return format!("You now have {} tokens.", tokens);
            }
        },
        Err(err) => {
            return err.to_string();
        }
    }
}

pub fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;
    let qty = item_quantity.parse::<i32>()?;

    Ok(qty * cost_per_item + processing_fee)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ok_case() {
        assert_eq!(buy_items(100, "8"), String::from("You now have 59 tokens."));
    }

    #[test]
    fn test_too_many_case() {
        assert_eq!(buy_items(100, "100"), String::from("You can't afford that many!"));
    }

    #[test]
    fn test_invalid_case() {
        assert_eq!(buy_items(100, "abcd"), String::from("invalid digit found in string"));
    }
}
// Since the `?` operator returns an `Err` early if the thing it's trying to
// do fails, you can only use the `?` operator in functions that have a
// `Result` as their return type.

// Hence the error that you get if you run this code is:

// ```
// error[E0277]: the `?` operator can only be used in a function that returns `Result` (or another type that implements `std::ops::Try`)
// ```

// So we have to use another way of handling a `Result` within `main`.

// Decide what we should do if `pretend_user_input` has a string value that does
// not parse to an integer, and implement that instead of using the `?`
// operator.
