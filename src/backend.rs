
pub struct Backend{
    pub hostname:String,
    pub port:i32,
}
impl Backend {
    pub fn new(hostname:String, port:i32) -> Backend {
        return Backend{
            hostname,
            port,
        }
    }
}
