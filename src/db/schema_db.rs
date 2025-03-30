use regex::Regex;
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Clone)]
pub struct FieldDefinition {
    pub name: String,
    pub field_type: String, // e.g., "option<string>", "record", etc.
                            // Add other relevant properties if needed (constraints, etc.)
}

#[derive(Debug, Clone)]
pub struct TableDefinition {
    pub name: String,
    pub fields: HashMap<String, FieldDefinition>, // Field name -> FieldDefinition
}

#[derive(Debug, Clone)]
pub struct Schema {
    pub tables: HashMap<String, TableDefinition>, // Table name -> TableDefinition
}

impl Schema {
    pub fn from_file(path: &str) -> Result<Self, String> {
        let contents = fs::read_to_string(path).map_err(|e| e.to_string())?;
        Schema::from_surql(&contents)
    }

    pub fn from_surql(surql: &str) -> Result<Self, String> {
        let mut tables: HashMap<String, TableDefinition> = HashMap::new();

        // Regex for table definition
        let table_regex = Regex::new(r"DEFINE TABLE IF NOT EXISTS (\w+)").unwrap();
        // Regex for field definition  --  Crucially, this handles spaces in types
        let field_regex = Regex::new(
            r"DEFINE FIELD IF NOT EXISTS (\w+) ON (\w+) TYPE ([\w<>, ]+)", // Capture field name, table name, and type
        )
        .unwrap();

        // Extract table definitions
        for cap in table_regex.captures_iter(surql) {
            let table_name = cap.get(1).unwrap().as_str().to_string();
            tables.insert(
                table_name.clone(),
                TableDefinition {
                    name: table_name,
                    fields: HashMap::new(),
                },
            );
        }

        // Extract field definitions
        for cap in field_regex.captures_iter(surql) {
            let field_name = cap.get(1).unwrap().as_str().to_string();
            let table_name = cap.get(2).unwrap().as_str().to_string();
            let field_type = cap.get(3).unwrap().as_str().trim().to_string(); // Trim whitespace

            if let Some(table) = tables.get_mut(&table_name) {
                table.fields.insert(
                    field_name.clone(),
                    FieldDefinition {
                        name: field_name,
                        field_type,
                    },
                );
            } else {
                // Handle the case where the field refers to a table not defined (error/warning).
                return Err(format!(
                    "Field '{}' refers to undefined table '{}'",
                    field_name, table_name
                ));
            }
        }

        println!("Tables: {:?}", &tables);

        Ok(Schema { tables })
    }

    // pub fn get_table(&self, table_name: &str) -> Option<&TableDefinition> {
    //     self.tables.get(table_name)
    // }

    // pub fn get_field(&self, table_name: &str, field_name: &str) -> Option<&FieldDefinition> {
    //     self.tables.get(table_name)?.fields.get(field_name)
    // }
}
