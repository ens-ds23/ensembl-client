use composit::SourceResponseData;

pub trait SourceResponse {
    fn set_state(&self, state: bool);
    fn check(&self) -> bool;
    fn refresh(&mut self);
    fn set(&mut self, result: SourceResponseData);
    fn source_response_clone(&self) -> Box<SourceResponse>;
    fn destroy(&mut self);
}
