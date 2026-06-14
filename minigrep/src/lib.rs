/*pub fn search<'a>(
    query: &str,
    contents: &'a str,
) -> Vec<&'a str> {

    let mut results = Vec::new();

    for line in contents.lines() {

        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {

        let query = "duct";

        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }
}*/

// Performs a case-sensitive search
pub fn search<'a>(
    query: &str,
    contents: &'a str,
) -> Vec<&'a str> {

    // Vector to store matching lines
    let mut results = Vec::new();

    // Iterate over each line in the contents
    for line in contents.lines() {

        // Check whether the line contains the query
        if line.contains(query) {

            // Add the matching line to the results vector
            results.push(line);
        }
    }

    // Return all matching lines
    results
}

// Performs a case-insensitive search
pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str,
) -> Vec<&'a str> {

    // Convert the query to lowercase
    let query = query.to_lowercase();

    // Vector to store matching lines
    let mut results = Vec::new();

    // Iterate through every line
    for line in contents.lines() {

        // Convert the line to lowercase and compare
        if line.to_lowercase().contains(&query) {

            // Store the original line
            results.push(line);
        }
    }

    // Return all matching lines
    results
}

#[cfg(test)]
mod tests {

    use super::*;

    // Test for case-sensitive searching
    #[test]
    fn case_sensitive() {

        let query = "duct";

        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        // Only the lowercase "duct" should match
        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }

    // Test for case-insensitive searching
    #[test]
    fn case_insensitive() {

        let query = "rUsT";

        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        // Both lines should match regardless of case
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}