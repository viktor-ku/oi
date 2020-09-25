use lib_api::{self as api, Client};
use pretty_assertions::assert_eq;

#[tokio::test]
async fn very_happy_flow() {
    let client = Client::new("http://localhost:8080");

    should_not_find_anything_at_all(&client).await;
    should_create_new_record(&client).await;
    should_find_one_record(&client).await;
    that_one_record_should_be_active(&client).await;

    tokio::time::delay_for(tokio::time::Duration::from_secs(3)).await;

    should_not_find_any_more_active_timers_after_3_secs(&client).await;
    should_find_one_record(&client).await;
}

async fn should_not_find_any_more_active_timers_after_3_secs(client: &Client<'_>) {
    println!("Should not find any active records anymore");

    let res = client
        .timers
        .find_active()
        .await
        .expect("Could not make a request!");

    assert_eq!(
        res.status().as_u16(),
        200,
        "Return code of find_all is not OK"
    );
    assert_eq!(res.data.unwrap().timers.is_empty(), true);
}

async fn that_one_record_should_be_active(client: &Client<'_>) {
    println!("Should also find one active record");

    let res = client
        .timers
        .find_active()
        .await
        .expect("Could not make a request!");

    assert_eq!(
        res.status().as_u16(),
        200,
        "Return code of find_all is not OK"
    );
    assert_eq!(res.data.unwrap().timers.len(), 1);
}

async fn should_find_one_record(client: &Client<'_>) {
    println!("Should find one record in total");

    let res = client
        .timers
        .find_all()
        .await
        .expect("Could not make a request!");

    assert_eq!(
        res.status().as_u16(),
        200,
        "Return code of find_all is not OK"
    );
    assert_eq!(res.data.unwrap().timers.len(), 1);
}

async fn should_not_find_anything_at_all(client: &Client<'_>) {
    println!("Should not find anything at all");

    let res = client
        .timers
        .find_all()
        .await
        .expect("Could not make a request!");

    assert_eq!(
        res.status().as_u16(),
        200,
        "Return code of find_all is not OK"
    );
    assert_eq!(res.data.unwrap().timers.is_empty(), true);
}

async fn should_create_new_record(client: &Client<'_>) {
    println!("Should create new record");

    let res = client
        .timers
        .create(&api::timer::CreateTimer {
            start: chrono::Utc::now().timestamp_millis(),
            duration: 3_000,
            message: "lets try 3 seconds simple timer for now".to_owned(),
        })
        .await
        .expect("Could not make a request!");

    assert_eq!(res.status().is_success(), true);
    assert_eq!(res.data.unwrap().uuid.as_bytes().is_empty(), false);
}
