use redis::Client;

#[derive(Clone, Debug)]
pub struct State {
    pub db: Client,
}

impl State {
    pub(crate) async fn new(uri: &str) -> tide::Result<Self> {
        let redis = Client::open(uri)?;
        Ok(Self { db: redis })
    }
}
