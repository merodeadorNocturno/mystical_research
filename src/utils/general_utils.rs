use rand::{rng, Rng};
use surrealdb::Uuid;
use unicode_normalization::{char::is_combining_mark, UnicodeNormalization};

pub fn get_uuid() -> String {
    let uuid_v7 = Uuid::now_v7();

    String::from(uuid_v7)
}

pub fn create_or_conditional(search_term: &str, fields: Vec<String>) -> String {
    let mut or_conditional = String::from("");
    for field in fields {
        if or_conditional == "".to_string() {
            or_conditional = format!("{field} @@ '{search_term}'");
        } else {
            or_conditional = format!("{or_conditional} OR {field} @@ '{search_term}'");
        }
    }

    or_conditional
}

pub fn trim_to_char_slice(s: &str, max_chars: usize) -> &str {
    match s.char_indices().nth(max_chars) {
        // If the character exists, `idx` is its starting byte index.
        // Slice up to that index.
        Some((idx, _)) => &s[..idx],
        // If the character doesn't exist (string is shorter than max_chars),
        // return the whole string slice.
        None => s,
    }
}

pub fn trim_to_words(s: &str, num_words: usize) -> String {
    // 1. Split the string into an iterator of subslices separated by whitespace.
    //    `split_whitespace()` handles multiple spaces, leading/trailing spaces gracefully.
    s.split_whitespace()
        // 2. Take only the first `num_words` items (words) from the iterator.
        //    If there are fewer than `num_words`, it takes all of them.
        .take(num_words)
        // 3. Collect the taken words into a Vec<&str>.
        //    This is necessary because `join` works on slices.
        .collect::<Vec<&str>>()
        // 4. Join the elements of the vector back into a single String,
        //    inserting a single space (" ") between each word.
        .join(" ")
}

pub fn generate_slug_with_random_suffix(title: &str) -> String {
    let normalized: String = title.nfd().filter(|c| !is_combining_mark(*c)).collect();
    let lowercased = normalized.to_lowercase();

    let mut slug_base = String::with_capacity(lowercased.len());
    let mut last_char_was_underscore = true; // Avoid leading underscore

    for c in lowercased.chars() {
        if c.is_alphanumeric() {
            slug_base.push(c);
            last_char_was_underscore = false;
        } else if !last_char_was_underscore && (c.is_whitespace() || "-.,:;!?()[]{}'".contains(c)) {
            // Replace whitespace and common punctuation with a *single* underscore
            slug_base.push('_');
            last_char_was_underscore = true;
        }
        // Ignore other characters or consecutive separators
    }

    // 4. Trim trailing underscore if present
    if slug_base.ends_with('_') {
        slug_base.pop();
    }

    // Handle cases where the title results in an empty slug (e.g., "!!!")
    if slug_base.is_empty() {
        slug_base.push_str("untitled");
    }

    // 5. Generate random suffix (using thread_rng and gen_range)
    let mut rng = rng(); // Get thread-local RNG
                         // Use gen_range for generating a random number within a range
    let random_suffix: u32 = rng.random_range(1000..=9999);

    // 6. Format the final slug string
    format!("{}_{}.html", slug_base, random_suffix)
}

pub fn string_to_vec_string(cs_string: String) -> Vec<String> {
    cs_string.split(',').map(|s| s.to_string()).collect()
}
