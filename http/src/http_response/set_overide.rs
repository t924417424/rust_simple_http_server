pub trait StateCode<T> {
    fn set_http_state_code(&mut self, state_code: T) -> &mut Self;
}