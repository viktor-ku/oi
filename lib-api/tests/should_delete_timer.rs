use lib_api::{self as api, Client};
use pretty_assertions::assert_eq;

#[tokio::test]
async fn should_delete_the_timer() {
    let client = Client::new("http://localhost:9999");

    let uuid = create_new_timer(&client).await;
    should_find_timer(&client, &uuid).await;
    should_delete_timer(&client, &uuid).await;
    tokio::join!(
        should_not_delete_already_deleted_timer(&client, &uuid),
        should_not_find_timer(&client, &uuid),
    );
}

async fn create_new_timer(client: &Client<'_>) -> uuid::Uuid {
    println!("Should create new timer");

    let res = client
        .timers
        .create(&api::timer::CreateTimerInput {
            start: chrono::Utc::now().timestamp_millis(),
            duration: 1_000 * 60 * 60 * 3,
            message: "long timer 3h".to_owned(),
        })
        .await
        .expect("Could not make a request!");

    assert_eq!(res.status().as_u16(), 200);

    res.data.unwrap().uuid
}

async fn should_find_timer(client: &Client<'_>, uuid: &uuid::Uuid) {
    println!("Should find timer {}", uuid);

    let res = client
        .timers
        .find_by_uuid(uuid)
        .await
        .expect("Could not make a request!");

    assert_eq!(res.status().as_u16(), 200);
    assert_eq!(
        res.data
            .unwrap()
            .timer
            .expect("Could not find a timer")
            .uuid,
        *uuid
    );
}

async fn should_not_find_timer(client: &Client<'_>, uuid: &uuid::Uuid) {
    println!("Should not find a timer {}", uuid);

    let res = client
        .timers
        .find_by_uuid(uuid)
        .await
        .expect("Could not make a request!");

    assert_eq!(res.status().as_u16(), 200);
    assert_eq!(res.data.unwrap().timer, None);
}

async fn should_delete_timer(client: &Client<'_>, uuid: &uuid::Uuid) {
    println!("Should delete timer {}", uuid);

    let res = client
        .timers
        .delete_by_uuid(uuid)
        .await
        .expect("Could not make a request!");

    assert_eq!(res.status().as_u16(), 200);
    assert_eq!(res.data.unwrap().uuid.unwrap(), *uuid);
}

async fn should_not_delete_already_deleted_timer(client: &Client<'_>, uuid: &uuid::Uuid) {
    println!("Should not make sense to delete a timer {} twice", uuid);

    let res = client
        .timers
        .delete_by_uuid(uuid)
        .await
        .expect("Could not make a request!");

    assert_eq!(res.status().as_u16(), 200);
    assert_eq!(res.data.unwrap().uuid, None);
}
