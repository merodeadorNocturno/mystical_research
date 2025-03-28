use super::schema_db::Schema;
use crate::config::connection::set_environment_variable;
use actix_web::web::Data;
use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    sql::Value,
    Error, Surreal,
};

#[derive(Debug)]
pub struct Database {
    pub client: Surreal<Client>,
    pub name_space: String,
    pub db_name: String,
    pub schema: Schema, // Add the schema here
}

impl Database {
    pub async fn init() -> Result<Self, Error> {
        let db_address: String = set_environment_variable("DB_ADDRESS", "0.0.0.0:8000");
        let db_ns = set_environment_variable("DB_NAMESPACE", "mystical_ns");
        let db_name = set_environment_variable("DB_NAME", "mystical_db");

        let client = Surreal::new::<Ws>(db_address).await?;

        client
            .signin(Root {
                username: &set_environment_variable("DB_USERNAME", "mystical_admin"),
                password: &set_environment_variable("DB_PASSWORD", "mystical_password"),
            })
            .await?;

        client.use_ns(&db_ns).use_db(&db_name).await.unwrap();

        let schema_path = "static/db/schema.surql";
        let schema = Schema::from_file(schema_path).map_err(|e| {
            // Convert the schema loading error to a SurrealDB Error
            eprintln!(
                "Convert the schema loading error to a SurrealDB Error {}",
                e
            );
            Error::Api(surrealdb::error::Api::Query(e)) // Or another appropriate Error variant
        })?;

        let schema_content = std::fs::read_to_string(schema_path).map_err(|e| {
            // Convert the schema loading error to a SurrealDB Error
            eprintln!(
                "Convert the schema loading error to a SurrealDB Error {}",
                e
            );
            Error::Api(surrealdb::error::Api::Query(e.to_string())) // Or another appropriate Error variant
        })?;

        // Execute the schema definition against the database.
        let response = client.query(schema_content).await?;
        if let Err(err) = response.check() {
            eprintln!("Error applying schema definition: {}", err);
            return Err(err);
        }
        println!("Schema applied successfully");

        Ok(Database {
            client,
            name_space: String::from(&db_ns),
            db_name: String::from(&db_name),
            schema, // Store the schema
        })
    }
}
