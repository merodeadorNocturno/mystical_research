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
use tracing::info;

#[derive(Debug)]
pub struct Database {
    pub client: Surreal<Any>,
    pub name_space: String,
    pub db_name: String,
    #[allow(dead_code)]
    pub schema: Schema, // Add the schema here
}

impl Database {
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
        // let client = Surreal::new::<Ws>(db_address).await?;
        // let client = any::connect(db_address).await?;

        // let client = match any::connect(format!("{}/rpc", &db_address)).await {
        //     Ok(c) => {
        //         println!("SUCCESS: Connected to database at {}", db_address);
        //         c
        //     }
        //     Err(e) => {
        //         eprintln!(
        //             "FAILURE: Could not connect to {} - Error: {}",
        //             db_address, e
        //         );
        //         return Err(e);
        //     }
        // };

        // client
        //     .signin(Root {
        //         username: &set_environment_variable("DB_USERNAME", "mystical_admin"),
        //         password: &set_environment_variable("DB_PASSWORD", "mystical_password"),
        //     })
        //     .await?;
        //
        // let username = set_environment_variable("DB_USERNAME", "mystical_admin");
        // let password = set_environment_variable("DB_PASSWORD", "mystical_password");

        // if let Err(e) = client
        //     .signin(Root {
        //         username: username.clone(),
        //         password: password,
        //     })
        //     .await
        // {
        //     eprintln!(
        //         "FAILURE: Authentication failed for user '{}' - Error: {}",
        //         &username, e
        //     );
        //     return Err(e);
        // }
        // println!("SUCCESS: Authenticated as {}", username);

        // client.use_ns(&db_ns).use_db(&db_name).await.unwrap();
        // if let Err(e) = client.use_ns(&db_ns).use_db(&db_name).await {
        //     eprintln!(
        //         "FAILURE: Failed to switch to NS: '{}', DB: '{}' - Error: {}",
        //         db_ns, db_name, e
        //     );
        //     return Err(e);
        // }
        // println!("SUCCESS: Using Namespace: {}, DB: {}", db_ns, db_name);

        let schema_path = "static/db/schema.surql";
        let schema = match Schema::from_file(schema_path) {
            Ok(s) => s,
            Err(e) => {
                eprintln!(
                    "CRITICAL: Schema helper failed to load path {} | Error: {}",
                    schema_path, e
                );
                // Adjusting the error mapping to match your expected return type
                // return Err(Error::Api(surrealdb::error::Api::Query(e.to_string())));
                return Err(Error::internal(e.to_string()));
            }
        };
        // let schema = Schema::from_file(schema_path).map_err(|e| {
        //     // Convert the schema loading error to a SurrealDB Error
        //     eprintln!(
        //         "Convert the schema loading error to a SurrealDB Error {}",
        //         e
        //     );
        //     Error::Api(surrealdb::error::Api::Query(e)) // Or another appropriate Error variant
        // })?;

        // let schema_content = std::fs::read_to_string(schema_path).map_err(|e| {
        //     // Convert the schema loading error to a SurrealDB Error
        //     eprintln!(
        //         "Convert the schema loading error to a SurrealDB Error {}",
        //         e
        //     );
        //     Error::Api(surrealdb::error::Api::Query(e.to_string())) // Or another appropriate Error variant
        // })?;

        let schema_content = match std::fs::read_to_string(schema_path) {
            Ok(content) => content,
            Err(e) => {
                eprintln!(
                    "FAILURE: Could not read schema file at {} - Error: {}",
                    schema_path, e
                );
                // Converting std::io::Error to SurrealDB Error
                // return Err(Error::Api(surrealdb::error::Api::Query(e.to_string())));
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
