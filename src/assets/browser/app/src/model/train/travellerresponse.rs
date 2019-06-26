use super::TravellerResponseData;

pub trait TravellerResponse {
    fn set_state(&self, state: bool);
    fn check(&self) -> bool;
    fn refresh(&mut self);
    fn set_response(&mut self, result: TravellerResponseData);
    fn source_response_clone(&self) -> Box<TravellerResponse>;
    fn destroy(&mut self);
}
