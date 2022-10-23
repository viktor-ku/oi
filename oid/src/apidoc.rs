use crate::app::*;
use lib_store::{Timer, TimerInput};
use utoipa::OpenApi;

#[derive(Debug, OpenApi)]
#[openapi(
    paths(
        create_timer,
        find_all_timers,
        delete_by_uuid,
        find_active_timers,
        find_by_uuid
    ),
    components(schemas(Timer, TimerInput))
)]
pub struct ApiDoc;
