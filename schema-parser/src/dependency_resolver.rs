//use std::collections::HashMap;
//
//use serde_json::Value;
//
//use rest_client::RestClient;
//
//use crate::configuration::Configuration;
//use crate::rest_endpoint::Endpoint;
//
//pub fn create_item(
//    endpoint: &Endpoint,
//    created_items: &mut HashMap<String, Value>,
//    configuration: &Configuration,
//    client: &RestClient,
//) {
//    for dependency in &endpoint.requires {
//        if !created_items.contains_key(dependency) {
//            create_item(&configuration.endpoints[dependency], created_items, configuration, client);
//        }
//    }
//
//    let response = client.post(&endpoint.url, &endpoint.default_payload());
//    match response {
//        None => panic!(format!("Failed to create endpoint {}. Stopping here!", endpoint.name)),
//        Some(_) => created_items[endpoint.name.clone()] = response.unwrap(),
//    }
//}
