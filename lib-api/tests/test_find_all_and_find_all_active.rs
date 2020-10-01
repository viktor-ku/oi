use lib_api::{self as api, Client};
use pretty_assertions::assert_eq;
use std::collections::HashSet;
use uuid::Uuid;

#[tokio::test]
async fn test_find_all_and_find_all_active() {
    let client = Client::new("http://localhost:9999");

    let (short_timer_uuids, long_timer_uuids) =
        tokio::join!(create_short_timers(&client), create_long_timers(&client));

    let all_timer_uuids = {
        let mut v = short_timer_uuids.clone();
        v.extend(long_timer_uuids);
        v
    };

    tokio::join!(
        should_find_timers(&client, &all_timer_uuids),
        should_find_active_timers(&client, &short_timer_uuids),
    );

    println!("Sleeping for 1s to wait until active timers timeout...");
    tokio::time::delay_for(tokio::time::Duration::from_secs(1)).await;

    tokio::join!(
        should_find_timers(&client, &all_timer_uuids),
        should_not_find_active_timers(&client, &short_timer_uuids),
    );
}

async fn should_find_timers(client: &Client<'_>, uuids: &[Uuid]) {
    println!("Should find all timers");

    let res = client
        .timers
        .find_all()
        .await
        .expect("Could not make a request!");

    let all_timers = res.data.unwrap().timers;
    let all_uuids = {
        let mut set: HashSet<Uuid> = HashSet::with_capacity(all_timers.len());

        for timer in &all_timers {
            set.insert(timer.uuid);
        }

        set
    };

    for uuid in uuids {
        assert_eq!(all_uuids.contains(uuid), true);
    }
}

async fn should_find_active_timers(client: &Client<'_>, uuids: &[Uuid]) {
    println!("Should find active timers");

    let res = client
        .timers
        .find_active()
        .await
        .expect("Could not make a request!");

    let all_timers = res.data.unwrap().timers;
    let all_uuids = {
        let mut set: HashSet<Uuid> = HashSet::with_capacity(all_timers.len());

        for timer in &all_timers {
            set.insert(timer.uuid);
        }

        set
    };

    for uuid in uuids {
        assert_eq!(all_uuids.contains(uuid), true);
    }
}

async fn should_not_find_active_timers(client: &Client<'_>, uuids: &[Uuid]) {
    println!("Should not find active timers");

    let res = client
        .timers
        .find_active()
        .await
        .expect("Could not make a request!");

    let all_timers = res.data.unwrap().timers;
    let all_uuids = {
        let mut set: HashSet<Uuid> = HashSet::with_capacity(all_timers.len());

        for timer in &all_timers {
            set.insert(timer.uuid);
        }

        set
    };

    for uuid in uuids {
        assert_eq!(all_uuids.contains(uuid), false);
    }
}

async fn create_timer(client: &Client<'_>, duration: u64) -> Uuid {
    let res = client
        .timers
        .create(&api::timer::CreateTimerInput {
            start: chrono::Utc::now().timestamp_millis(),
            duration,
            message: format!("this is a test {}s timer", duration / 1_000),
        })
        .await
        .expect("Could not make a request!");

    assert_eq!(res.status().as_u16(), 200);
    res.data.unwrap().uuid
}

#[inline]
async fn create_short_timer(client: &Client<'_>) -> Uuid {
    create_timer(client, 1_000).await
}
#[inline]
async fn create_long_timer(client: &Client<'_>) -> Uuid {
    create_timer(client, 3_600_000).await
}

async fn create_short_timers(client: &Client<'_>) -> Vec<Uuid> {
    let uuids = tokio::join!(
        create_short_timer(&client),
        create_short_timer(&client),
        create_short_timer(&client),
        create_short_timer(&client),
        create_short_timer(&client),
    );
    vec![uuids.0, uuids.1, uuids.2, uuids.3, uuids.4]
}
async fn create_long_timers(client: &Client<'_>) -> Vec<Uuid> {
    let uuids = tokio::join!(
        create_long_timer(&client),
        create_long_timer(&client),
        create_long_timer(&client),
        create_long_timer(&client),
        create_long_timer(&client),
    );
    vec![uuids.0, uuids.1, uuids.2, uuids.3, uuids.4]
}
