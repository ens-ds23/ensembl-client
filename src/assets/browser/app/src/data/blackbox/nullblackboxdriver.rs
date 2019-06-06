use super::BlackBoxDriverImpl;
use url::Url;

pub struct NullBlackBoxDriverImpl {}

impl NullBlackBoxDriverImpl {
    pub fn new() -> NullBlackBoxDriverImpl {
        NullBlackBoxDriverImpl {}
    }
}

impl BlackBoxDriverImpl for NullBlackBoxDriverImpl {
}
