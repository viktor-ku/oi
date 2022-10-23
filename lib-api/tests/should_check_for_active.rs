use lib_api::{self as api, Client};
use pretty_assertions::assert_eq;

#[tokio::test]
async fn should_check_for_active() {
    let client = Client::new("http://localhost:9999");

    let uuid = create_new_timer(&client).await;

    should_find_active_timer(&client, &uuid).await;

    println!("Sleeping for 3 seconds to wait until active timers timeout...");
    tokio::time::delay_for(tokio::time::Duration::from_secs(3)).await;

    should_find_inactive_timer(&client, &uuid).await;
}

async fn create_new_timer(client: &Client<'_>) -> uuid::Uuid {
    println!("Should create new timer");

    let res = client
        .timers
        .create(&api::timer::CreateTimerInput {
            start: chrono::Utc::now().timestamp_millis(),
            duration: 3_000,
            message: "3 secs simple timer".to_owned(),
        })
        .await
        .expect("Could not make a request!");

    assert_eq!(res.status().as_u16(), 200);
    res.data.unwrap().uuid
}

async fn should_find_active_timer(client: &Client<'_>, uuid: &uuid::Uuid) {
    println!("Should find an active timer {}", uuid);

    let res = client
        .timers
        .find_by_uuid(uuid)
        .await
        .expect("Could not make a request!");

    assert_eq!(res.status().as_u16(), 200);

    let timer = res.data.unwrap().timer.unwrap();
    assert_eq!(timer.uuid, *uuid, "uuid should match");
    assert_eq!(timer.is_active(None), true, "timer should be active");
}

async fn should_find_inactive_timer(client: &Client<'_>, uuid: &uuid::Uuid) {
    println!("Should find inactive timer {}", uuid);

    let res = client
        .timers
        .find_by_uuid(uuid)
        .await
        .expect("Could not make a request!");

    assert_eq!(res.status().as_u16(), 200);

    let timer = res.data.unwrap().timer.unwrap();
    assert_eq!(timer.uuid, *uuid, "uuid should match");
    assert_eq!(timer.is_active(None), false, "timer should not be active");
}
