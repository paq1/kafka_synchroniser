pub trait CanGetCorrelationId {
    fn get_correlation_id(&self) -> String;
}