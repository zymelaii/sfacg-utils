pub trait AuthApi {
    fn is_authenticated(&self) -> bool;

    fn login(&mut self, account: &str, password: &str) -> Option<String>;

    fn logout(&mut self) -> bool;
}
