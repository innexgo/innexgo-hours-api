use reqwest::Client;

#[derive(Clone)]
pub struct InnexgoHoursService {
  client: Client,
  innexgo_hours_service_url: String,
}

impl InnexgoHoursService {
  pub async fn new(innexgo_hours_service_url: &str) -> Self {
    InnexgoHoursService {
      innexgo_hours_service_url: String::from(innexgo_hours_service_url),
      client: Client::new(),
    }
  }
}
