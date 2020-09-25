mod response;
pub use response::Response;

mod resources;
pub use resources::timer;

#[derive(Debug)]
pub struct Client<'api> {
    base_url: &'api str,
    pub timers: timer::TimersResource<'api>,
}

impl<'api> Client<'api> {
    pub fn new(base_url: &'api str) -> Self {
        Self {
            base_url,
            timers: timer::TimersResource::new(base_url),
        }
    }
}
