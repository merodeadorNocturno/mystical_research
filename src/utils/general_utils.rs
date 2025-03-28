use surrealdb::Uuid;

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
