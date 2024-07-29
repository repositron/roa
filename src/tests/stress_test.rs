use reqwest::Client;
use serde_json::json;
use tokio::task;
use uuid::Uuid;

use crate::routes;
//use crate::schema::orders::table_id;

async fn post_order(client: &Client, uuid: &Uuid) -> Result<(), reqwest::Error> {
    let payload = json!({
        "id": uuid,
        "tableId": 55,
        "item": "food",
    });

    let response = client.post("http://localhost:3000/orders")
        .json(&payload)
        .send()
        .await?;

    Ok(()) // Return Ok on success
}

async fn get_order(client: &Client, table_id: i32) -> Result<(), reqwest::Error> {
    let response = client.get(format!("http://localhost:3000/orders/{table_id}"))
        .send()
        .await?;

    Ok(())
}

async fn delete_order(client: &Client, uuid: &Uuid) -> Result<(), reqwest::Error> {
    let response = client.delete(format!("http://localhost:3000/orders/{uuid}"))
        .send()
        .await?;

    Ok(())
}

#[tokio::test]
async fn stress_test() {
    let num_requests = 100;

    let client = Client::new();

    let mut new_order_tasks = Vec::new();

    let uuids = Vec::from_iter((0..num_requests).map(|_| Uuid::new_v4()));

    for uuid in uuids.clone() {
        let client_clone = client.clone();
        let task_handle = task::spawn(async move {
            post_order(&client_clone, &uuid).await.unwrap_or_else(|e| {
                eprintln!("Error in request: {:?}", e);
            });
        });
        new_order_tasks.push(task_handle);
    }

    for task in new_order_tasks {
        task.await.unwrap();
    }

    let mut get_order_tasks = Vec::new();

    for _ in 0..5 {
        let client_clone = client.clone();
        let task_handle = task::spawn(async move {
            get_order(&client_clone, 55).await.unwrap_or_else(|e| {
                eprintln!("Error in request: {:?}", e);
            });
        });
        get_order_tasks.push(task_handle);
    }

    for task in get_order_tasks {
        task.await.unwrap();
    }

    let mut delete_order_tasks = Vec::new();

    for uuid in uuids {
        let client_clone = client.clone();
        let task_handle = task::spawn(async move {
            delete_order(&client_clone, &uuid).await.unwrap_or_else(|e| {
                eprintln!("Error in request: {:?}", e);
            });
        });
        delete_order_tasks.push(task_handle);
    }


    for task in delete_order_tasks {
        task.await.unwrap();
    }

}
