use crate::db::config_db::Database;
// use crate::models::general_model::*;
use crate::utils::general_utils::get_uuid;
use actix_web::web::Data;
// use chrono::Local;
use log::error;
// use reqwest::Response;
use crate::models::general_model::CountResult;
use serde::{de::DeserializeOwned, Serialize};
use surrealdb::{Error, Response as SR_Response};

// pub async fn util_find_all<T: DeserializeOwned>(
//     db: &Data<Database>,
//     table_name: &str,
// ) -> Option<Vec<T>> {
//     let result = db.client.select(table_name).await;

//     match result {
//         Ok(all_results) => Some(all_results),
//         Err(err) => {
//             error!(
//                 "Error fetching all records from table {}: {}",
//                 table_name, err
//             );
//             None
//         }
//     }
// }

pub async fn util_find_one<T: DeserializeOwned>(
    db: &Data<Database>,
    id: String,
    table_name: &str,
) -> Option<T> {
    let result: Result<Option<T>, Error> = db.client.select((table_name, &id)).await;

    match result {
        Ok(record) => record,
        Err(err) => {
            error!(
                "Error fetching record with ID {} from table {}: {}",
                &id, &table_name, &err
            );
            None
        }
    }
}

pub async fn util_add_one<T>(db: &Data<Database>, record: T, table_name: &str) -> Option<T>
where
    T: DeserializeOwned + Serialize + Send + Sync + 'static,
{
    let uuid_v7 = get_uuid();
    let added_t_record = db
        .client
        .create((table_name, &uuid_v7))
        .content(record)
        .await;

    match added_t_record {
        Ok(added_record) => added_record,
        Err(err) => {
            error!(
                "Error adding record with ID {} to table {}: {}",
                &uuid_v7.to_string(),
                &table_name,
                &err
            );
            None
        }
    }
}

pub async fn util_update_record<T: DeserializeOwned + Serialize>(
    db: &Data<Database>,
    record: T,
    table_name: &str,
) -> Option<T>
where
    T: DeserializeOwned + Serialize + Send + Sync + 'static,
{
    let t_id = get_uuid();
    let t_to_update: Result<Option<T>, Error> =
        db.client.select((table_name, &t_id.to_string())).await;

    match t_to_update {
        Ok(t_found) => match t_found {
            Some(_t) => {
                let updated_t: Result<Option<T>, Error> = db
                    .client
                    .update((table_name, &t_id.to_string()))
                    .merge(record)
                    .await;

                match updated_t {
                    Ok(updated_t_values) => updated_t_values,
                    Err(err) => {
                        error!(
                            "Error updating record with ID {} in table {}: {}",
                            &t_id, &table_name, &err
                        );
                        None
                    }
                }
            }
            None => None,
        },
        Err(err) => {
            error!(
                "Error fetching record with ID {} in table {}: {}",
                &t_id, &table_name, &err
            );
            None
        }
    }
}

pub async fn util_find_active_records<T: DeserializeOwned + Serialize>(
    db: &Data<Database>,
    table_name: &str,
    number_of_records: Option<usize>,
) -> Option<Vec<T>> {
    let surreal_query = match number_of_records {
        Some(number) => format!(
            "SELECT * FROM {} WHERE deleted = false LIMIT {}",
            table_name, number
        ),
        None => format!("SELECT * FROM {} WHERE deleted = false", table_name),
    };
    let query_t_result = db.client.query(surreal_query).await;

    match query_t_result {
        Ok(mut result) => match result.take(0) {
            Ok(deleted_t_records) => Some(deleted_t_records),
            Err(err) => {
                error!("Error fetching records in table {}: {}", &table_name, &err);
                None
            }
        },
        Err(err) => {
            error!("Error executing query in table {}: {}", &table_name, &err);
            None
        }
    }
}

pub async fn util_fulltext_search<T: DeserializeOwned + Serialize>(
    db: &Data<Database>,
    table_name: &str,
    search_fields: &str,
) -> Option<Vec<T>> {
    let surreal_query = format!(
        "SELECT * FROM {table_name}
        WHERE {search_fields} AND deleted = false;"
    );
    let query_t_result = db.client.query(surreal_query).await;

    match query_t_result {
        Ok(mut result) => match result.take(0) {
            Ok(deleted_t_records) => Some(deleted_t_records),
            Err(err) => {
                error!("Error fetching records in table {}: {}", &table_name, &err);
                None
            }
        },
        Err(err) => {
            error!("Error executing query in table {}: {}", &table_name, &err);
            None
        }
    }
}

// --- NEW PAGINATED FUNCTION ---
/// Fetches a specific page of active (not deleted) records from a table.
///
/// # Arguments
/// * `db` - Database connection data.
/// * `table_name` - The name of the table to query.
/// * `page` - The desired page number (1-based index).
/// * `page_size` - The number of records per page.
/// * `order_by_field` - Optional field to order by (e.g., "created_at"). Defaults to "created_at".
/// * `descending` - Whether to sort in descending order. Defaults to true.
///
/// # Returns
/// * `Option<Vec<T>>` - A vector of records for the requested page, or None on error.
pub async fn util_find_active_paginated<T>(
    db: &Data<Database>,
    table_name: &str,
    page: usize,
    page_size: usize,
    order_by_field: Option<&str>,
    descending: Option<bool>,
) -> Option<Vec<T>>
where
    T: DeserializeOwned + Send + Sync + 'static, // Removed Serialize trait bound as it's not needed for reads
{
    // Basic validation for pagination parameters
    if page <= 0 || page_size <= 0 {
        error!(
            "Invalid pagination parameters for table {}: page={}, page_size={}. Page must be > 0, page_size must be > 0.",
            table_name, page, page_size
        );
        return None;
    }

    // Calculate the starting record index (0-based) for SurrealDB's START AT
    let start_index = (page - 1) * page_size;

    // Determine ordering
    let order_field = order_by_field.unwrap_or("created_at"); // Default order field
    let order_direction = if descending.unwrap_or(true) {
        "DESC"
    } else {
        "ASC"
    }; // Default descending

    // Construct the SurrealQL query with WHERE, ORDER BY, LIMIT, and START AT clauses
    let surreal_query = format!(
        "SELECT * FROM {} WHERE deleted = false ORDER BY {} {} LIMIT {} START {};",
        table_name, order_field, order_direction, page_size, start_index
    );

    // Execute the query
    let query_result = db.client.query(&surreal_query).await; // Pass query as reference

    match query_result {
        Ok(mut response) => {
            // Take the result from the first (and only) statement in the query
            match response.take(0) {
                Ok(records) => Some(records),
                Err(err) => {
                    error!(
                        "Error taking paginated results from table {}: {} (Query: {})",
                        table_name, err, surreal_query
                    );
                    None
                }
            }
        }
        Err(err) => {
            error!(
                "Error executing paginated query on table {}: {} (Query: {})",
                table_name, err, surreal_query
            );
            None
        }
    }
}
// --- END NEW PAGINATED FUNCTION ---

pub async fn util_count_records(db: &Data<Database>, table_name: &str) -> Option<u64> {
    let surreal_query = format!("SELECT count() FROM {} GROUP ALL", table_name);

    let index: usize = 0;
    let query_result = db.client.query(surreal_query.clone()).await;

    match query_result {
        Ok(mut result) => {
            let take_result: Result<Vec<CountResult>, Error> = result.take(index);
            match take_result {
                Ok(count_vec) => {
                    if let Some(count_obj) = count_vec.first() {
                        Some(count_obj.count)
                    } else {
                        error!(
                            "Count query for table {} returned empty result set unexpectedly (Query: {})",
                            table_name, &surreal_query
                        );
                        None
                    }
                }
                Err(err) => {
                    error!(
                        "Error taking count from table {}: {} (Query: {})",
                        table_name, err, surreal_query
                    );
                    None
                }
            }
        }
        Err(err) => {
            error!(
                "Error executing count query on table {}: {} (Query: {})",
                table_name, err, surreal_query
            );
            None
        }
    }
}
// --- END NEW COUNT FUNCTION ---

// pub async fn util_query_deleted<T: DeserializeOwned + Serialize>(
//     db: &Data<Database>,
//     table_name: &str,
// ) -> Option<Vec<T>> {
//     let surreal_query = format!("SELECT * FROM {} WHERE deleted = true", table_name);

//     let query_t_result: Result<SR_Response, Error> = db.client.query(surreal_query).await;

//     match query_t_result {
//         Ok(mut response) => match response.take(0) {
//             Ok(deleted_t_records) => Some(deleted_t_records),
//             Err(e) => {
//                 error!(
//                     "Failed to retrieve active records from {}:: {}",
//                     table_name, e
//                 );
//                 None
//             }
//         },
//         Err(e) => {
//             error!(
//                 "Failed to retrieve active records from {}:: {}",
//                 table_name, e
//             );
//             None
//         }
//     }
// }

// pub async fn util_remove_record<T: DeserializeOwned + Serialize>(
//     db: &Data<Database>,
//     id: String,
//     table_name: &str,
// ) -> Option<T>
// where
//     T: DeserializeOwned + Serialize + Send + Sync + 'static,
// {
//     let t_id = id.clone();
//     let t_to_update: Result<Option<T>, Error> = db.client.select((table_name, &t_id)).await;

//     match t_to_update {
//         Ok(t_found) => match t_found {
//             Some(_t) => {
//                 let updated_t: Result<Option<T>, Error> = db
//                     .client
//                     .update((table_name, &t_id))
//                     .merge(Deleted {
//                         deleted: true,
//                         updated_at: Local::now(),
//                     })
//                     .await;

//                 match updated_t {
//                     Ok(updated_t_values) => updated_t_values,
//                     Err(err) => {
//                         error!(
//                             "Error updating record with ID {} in table {}: {}",
//                             &t_id, &table_name, &err
//                         );
//                         None
//                     }
//                 }
//             }
//             None => None,
//         },
//         Err(err) => {
//             error!(
//                 "Error fetching record with ID {} in table {}: {}",
//                 &t_id, &table_name, &err
//             );
//             None
//         }
//     }
// }

pub async fn util_find_random_articles<T>(
    db: &Data<Database>,
    table_name: &str,
    number_of_elements: Option<usize>,
) -> Option<Vec<T>>
where
    T: DeserializeOwned + Serialize + Send + Sync + 'static,
{
    let surreal_string_query = match number_of_elements {
        Some(noe) => format!(
            "SELECT * FROM {} ORDER BY RAND() LIMIT {};",
            table_name, noe
        ),
        None => format!("SELECT * FROM {} ORDER BY RAND();", table_name),
    };

    let result: Result<SR_Response, Error> = db.client.query(surreal_string_query).await;

    match result {
        Ok(mut response) => match response.take(0) {
            Ok(resulting_articles) => Some(resulting_articles),
            Err(err) => {
                error!(
                    "Error fetching random articles from table {}: {}",
                    table_name, &err
                );
                None
            }
        },
        Err(err) => {
            error!(
                "Error fetching random articles from table {}: {}",
                table_name, &err
            );
            None
        }
    }
}
