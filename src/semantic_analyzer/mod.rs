pub mod resolver;
pub mod type_checker;

use crate::parser::Span;

/// Could be either a warning or an error
type Message = (Span, String);

/// Implements the Damerau-Levenshtein distance
pub fn edit_distance(x: &str, y: &str) -> usize {
    let x_chars = x.chars().collect::<Vec<char>>();
    let y_chars = y.chars().collect::<Vec<char>>();

    let x_len = x.chars().count();
    let y_len = y.chars().count();

    let mut dp = vec![vec![0; y_len + 1]; x_len + 1];

    for i in 0..=x_len {
        dp[i][0] = i;
    }

    for j in 0..=y_len {
        dp[0][j] = j;
    }

    for i in 1..=x_len {
        for j in 1..=y_len {
            let cost = if x_chars[i - 1] == y_chars[j - 1] {
                0
            } else {
                1
            };

            let deletion = dp[i - 1][j] + 1;
            let insertion = dp[i][j - 1] + 1;
            let substitution = dp[i - 1][j - 1] + cost;

            dp[i][j] = deletion.min(insertion).min(substitution);

            if i > 1
                && j > 1
                && x_chars[i - 1] == y_chars[j - 2]
                && x_chars[i - 2] == y_chars[j - 1]
            {
                dp[i][j] = dp[i][j].min(dp[i - 2][j - 2] + 1);
            }
        }
    }

    dp[x_len][y_len]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_damerau_levenshtein_distance() {
        assert_eq!(edit_distance("acb", "abc"), 1);
        assert_eq!(edit_distance("teh", "the"), 1);
        assert_eq!(edit_distance("apple", "aple"), 1);
        assert_eq!(edit_distance("aple", "apple"), 1);
        assert_eq!(edit_distance("abc", "acb"), 1);
    }
}
