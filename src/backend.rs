
pub struct ExecutableBackend {
    pub path:String,
    pub random_port:bool,
    pub default_port:i32,
}
pub struct PythonBackend{
    pub python_path:String,
    pub random_port:bool,
    pub python_main_path:String,
    pub default_port:i32,
}
pub trait Backend {
    fn get_start_command(&self,port:i32) -> String;
    fn is_random_port(&self) -> bool ;
    fn get_default_port(&self)->i32;
}
impl Backend for ExecutableBackend{
    fn get_start_command(&self,port:i32) -> String {
        return format!("start {}", self.path)
    }
    fn is_random_port(&self) -> bool {
        return self.random_port;
    }
    fn get_default_port(&self) -> i32 {
        return self.default_port;
    }
}
impl Backend for PythonBackend{
    fn get_start_command(&self,port:i32) -> String {
        return "".to_string()
    }
    fn is_random_port(&self) -> bool {
        return self.random_port;
    }
    fn get_default_port(&self) -> i32 {
        return self.default_port;
    }
}