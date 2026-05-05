use super::schema_db::Schema;
use crate::config::connection::set_environment_variable;
use surrealdb::{
    Error,
    Surreal,
    engine::{
        any::{self, Any},
        // remote::ws::{Client, Wss},
    }, //, remote::ws::Client},
    opt::auth::Root,
};


#[derive(Debug)]
pub struct Database {
    pub client: Surreal<Any>,
    pub name_space: String,
    pub db_name: String,
    #[allow(dead_code)]
    pub schema: Schema, // Add the schema here
}

impl Database {
    pub async fn authenticate(&self) -> Result<(), Error> {
        let db_ns = set_environment_variable("DB_NAMESPACE", "mystical_ns");
        let db_name = set_environment_variable("DB_NAME", "mystical_db");
        let username = set_environment_variable("DB_USERNAME", "mystical_admin");
        let password = set_environment_variable("DB_PASSWORD", "mystical_password");

        // Clear any expired session
        let _ = self.client.invalidate().await;

        self.client
            .signin(Root {
                username,
                password,
            })
            .await?;

        self.client.use_ns(&db_ns).use_db(&db_name).await?;
        Ok(())
    }

    pub async fn init() -> Result<Self, Error> {
        let db_address: String = set_environment_variable("DB_ADDRESS", "0.0.0.0:8000");
        let db_ns = set_environment_variable("DB_NAMESPACE", "mystical_ns");
        let db_name = set_environment_variable("DB_NAME", "mystical_db");

        let username = set_environment_variable("DB_USERNAME", "mystical_admin");
        let password = set_environment_variable("DB_PASSWORD", "mystical_password");

        // info!(
        //     "**** address: {}, ns: {}, name: {}, username: {}, password: {}",
        //     &db_address.to_string(),
        //     &db_ns.to_string(),
        //     &db_name.to_string(),
        //     &username,
        //     &password,
        // );

        let client = any::connect(format!("{}/rpc", &db_address)).await?;
        client
            .signin(Root {
                username: username.clone(),
                password: password,
            })
            .await?;
        client.use_ns(&db_ns).use_db(&db_name).await?;

        let schema_path = "static/db/schema.surql";
        let schema = match Schema::from_file(schema_path) {
            Ok(s) => s,
            Err(e) => {
                eprintln!(
                    "CRITICAL: Schema helper failed to load path {} | Error: {}",
                    schema_path, e
                );
                return Err(Error::internal(e.to_string()));
            }
        };

        let schema_content = match std::fs::read_to_string(schema_path) {
            Ok(content) => content,
            Err(e) => {
                eprintln!(
                    "FAILURE: Could not read schema file at {} - Error: {}",
                    schema_path, e
                );
                return Err(Error::internal(e.to_string()));
            }
        };

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
