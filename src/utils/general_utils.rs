use crate::utils::env_utils::PageConfiguration;
use log::info;
use rand::{rng, Rng};
use std::{fs, io, path::PathBuf};
use surrealdb::Uuid;
use unicode_normalization::{char::is_combining_mark, UnicodeNormalization};

pub fn get_uuid() -> String {
    let uuid_v7 = Uuid::now_v7();

    String::from(uuid_v7)
}

pub fn create_or_conditional(search_term: &str, fields: Vec<String>) -> String {
    let mut or_conditional = String::from("");
    for field in fields {
        if or_conditional == *"" {
            or_conditional = format!("{field} @@ '{search_term}'");
        } else {
            or_conditional = format!("{or_conditional} OR {field} @@ '{search_term}'");
        }
    }

    or_conditional
}

// pub fn trim_to_char_slice(full_string: &str, max_chars: usize) -> &str {
//     match full_string.char_indices().nth(max_chars) {
//         // If the character exists, `index` is its starting byte index.
//         // Slice up to that index.
//         Some((index, _)) => &full_string[..index],
//         // If the character doesn't exist (string is shorter than max_chars),
//         // return the whole string slice.
//         None => full_string,
//     }
// }

pub fn trim_to_words(full_string: &str, num_words: usize) -> String {
    // 1. Split the string into an iterator of subslices separated by whitespace.
    //    `split_whitespace()` handles multiple spaces, leading/trailing spaces gracefully.
    full_string
        .split_whitespace()
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

pub fn create_pagination_links(current_page: u64, total_pages: u64) -> String {
    let mut links = String::new();

    // Generate previous link
    if current_page > 1 {
        let cp = current_page - 1;
        links.push_str(&format!(
            "<a href=\"/blog_home.html?page={}\" hx-get=\"/htmx/blog?page={}\" hx-target=\".main_container\" hx-push-url=\"/blog_home.html?page={}\">Previous</a> ",
            cp, cp, cp
        ));
    }

    // Generate page numbers
    for page in 1..=total_pages {
        if page == current_page {
            links.push_str(&format!("<span class=\"current\">{}</span> ", page));
        } else {
            links.push_str(&format!("<a href=\"/blog_home.html?page={}\" hx-get=\"/htmx/blog?page={}\" hx-target=\".main_container\" hx-push-url=\"/blog_home.html?page={}\">{}</a> ", page, page, page, page));
        }
    }

    // Generate next link
    if current_page < total_pages {
        let cp = current_page + 1;
        links.push_str(&format!("<a href=\"/blog_home.html?page={}\" hx-get=\"/htmx/blog?page={}\" hx-target=\".main_container\" hx-push-url=\"/blog_home.html?page={}\">Next</a>", cp, cp, cp));
    }

    links
}

// Creates a robots.txt file in the static/templates directory.
///
/// This function generates a basic robots.txt allowing all crawlers access
/// and specifying the sitemap location.
///
/// # Arguments
///
/// * `template_dir` - The path to the `static/templates` directory.
/// * `base_url` - The base URL of the website (e.g., "https://mysticalresearch.com")
///              used for generating the Sitemap URL.
///
/// # Errors
///
/// Returns an `io::Error` if the directory cannot be created or the file
/// cannot be written.
pub fn create_robots_txt_template(output_dir: &PathBuf, base_url: &str) -> io::Result<()> {
    let robots_content = format!(
        r#"User-agent: *
Allow: /
# Disallow: /admin/ # Example: Disallow specific paths if needed
# Disallow: /private/

Sitemap: {}/sitemap.xml
"#,
        base_url.trim_end_matches('/') // Ensure no double slash in sitemap URL
    );

    let mut robots_path = output_dir.clone();
    robots_path.push("robots.txt");

    fs::write(&robots_path, robots_content)?;
    info!("Successfully created robots.txt at: {:?}", robots_path);
    Ok(())
}

/// Creates a basic sitemap.xml file in the specified directory.
///
/// This function generates a placeholder sitemap. A real implementation
/// should dynamically generate this based on website content (pages, blog posts).
///
/// # Arguments
///
/// * `output_dir` - The path to the directory where the sitemap should be created.
/// * `base_url` - The base URL of the website (e.g., "https://mysticalresearch.com").
///
/// # Errors
///
/// Returns an `io::Error` if the file cannot be written.
pub fn create_sitemap_xml_template(output_dir: &PathBuf, base_url: &str) -> io::Result<()> {
    // Ensure base_url ends with a slash for correct concatenation below
    let base = format!("{}/", base_url.trim_end_matches('/'));

    // Basic placeholder sitemap content.
    // TODO: Replace this with dynamic generation based on your content (blog posts, pages).
    let sitemap_content = format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
  <url>
    <loc>{}</loc>
    <lastmod>{}</lastmod>
    <changefreq>daily</changefreq>
    <priority>1.0</priority>
  </url>
  <url>
    <loc>{}about.html</loc>
    <changefreq>monthly</changefreq>
    <priority>0.8</priority>
  </url>
  <url>
    <loc>{}blog_home.html</loc>
    <changefreq>weekly</changefreq>
    <priority>0.9</priority>
  </url>
  <url>
    <loc>{}resources.html</loc>
    <changefreq>monthly</changefreq>
    <priority>0.7</priority>
  </url>
   <url>
    <loc>{}topics.html</loc>
    <changefreq>monthly</changefreq>
    <priority>0.7</priority>
  </url>
   <url>
    <loc>{}contact.html</loc>
    <changefreq>yearly</changefreq>
    <priority>0.5</priority>
  </url>
  <!-- Add other important URLs here -->
</urlset>
"#,
        base,
        chrono::Utc::now().format("%Y-%m-%d").to_string(), // Add a lastmod for the homepage
        base,
        base,
        base,
        base,
        base
    );

    let mut sitemap_path = output_dir.clone();
    sitemap_path.push("sitemap.xml");

    fs::write(&sitemap_path, sitemap_content)?;
    info!("Successfully created sitemap.xml at: {:?}", sitemap_path);
    Ok(())
}

// --- Helper function to get the designated static content directory ---
// You might already have this logic in env_utils, adjust as needed.
// This example assumes your robots.txt and sitemap.xml will live directly
// inside the `template_path` directory. If they should live elsewhere (e.g., a
// dedicated `public` or `static_root` directory), adjust accordingly.

/// Gets the configured path for static template files.
/// It's often better to determine this path once at startup.
pub fn get_template_path() -> PathBuf {
    // Example: Reading from an environment variable or using a default
    // Ensure this aligns with how `set_env_urls` determines `template_path`.
    let path_str =
        std::env::var("TEMPLATE_PATH").unwrap_or_else(|_| "static/templates".to_string());
    PathBuf::from(path_str)
}

/// Gets the base URL for the site.
/// Ensure this aligns with how `set_env_urls` determines the base URL.
pub fn get_base_url(page_config: &PageConfiguration) -> String {
    // Construct base URL from config. Handle default ports.
    let scheme = "http"; // Or "https" if using TLS
    let port_str = match (scheme, page_config.server_port.as_str()) {
        ("http", "80") => "".to_string(),
        ("https", "443") => "".to_string(),
        _ => format!(":{}", page_config.server_port),
    };
    format!("{}://{}{}", scheme, page_config.server_address, port_str)
}
