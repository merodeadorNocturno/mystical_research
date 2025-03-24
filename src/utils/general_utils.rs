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
