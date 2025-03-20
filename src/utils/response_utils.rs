use crate::models::ai_model::BlogStructure;

pub fn split_response(response: &str) -> Vec<String> {
    response
        .split('\n')
        .map(|s| s.to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

pub fn create_blog_structure_from_response(response: &str) -> BlogStructure {
    // println!("response: {:?}", response);
    let sections = split_response(response);
    let keywords = sections[sections.len() - 1]
        .split(',')
        .map(|s| s.trim().to_string())
        .collect();

    let table_of_contents = sections[1]
        .split(',')
        .map(|s| s.trim().to_string())
        .collect();

    // println!(" TOC .::{:?}::.", table_of_contents);

    BlogStructure {
        title: sections[0].clone(),
        table_of_contents,
        summary: sections[2].clone(),
        content: sections[3..sections.len() - 2]
            .iter()
            .map(|s| format!("<p>{}</p>", s.clone()))
            .collect(),
        keywords,
    }
}
