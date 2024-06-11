use crate::backend_starter::{BackendStarter, PythonBackendStarter};
pub struct ExecutableBackend {
    pub path:String,
    pub random_port:bool,
    pub default_port:i32,
    pub starter:Box<dyn BackendStarter>,
}
pub struct PythonBackend{
    pub python_path:String,
    pub random_port:bool,
    pub python_main_path:String,
    pub default_port:i32,
    pub starter:Box<dyn BackendStarter>,
}
pub trait Backend {
    fn is_random_port(&self) -> bool ;
    fn get_default_port(&self)->i32;
    fn get_starter(&self)->Box<dyn BackendStarter>;
}
impl Backend for PythonBackend{
    fn is_random_port(&self) -> bool {
        return self.random_port.clone();
    }
    fn get_default_port(&self) -> i32 {
        return self.default_port.clone();
    }
    fn get_starter(&self) -> Box<dyn BackendStarter> {
        return self.starter.clone();
    }
}
impl PythonBackend{
    pub fn new(python_path:String,python_main_path:String)->Self{
        return PythonBackend{
            python_path:python_path.clone(),
            python_main_path:python_main_path.clone(),
            random_port:true,
            default_port:8080,
            starter:Box::new(PythonBackendStarter{
                python_path:python_path.clone(),
                python_main_path:python_main_path.clone(),
            }),

        }
    }
}
impl Backend for ExecutableBackend{
    fn is_random_port(&self) -> bool {
        return self.random_port.clone();
    }
    fn get_default_port(&self) -> i32 {
        return self.default_port.clone();
    }
    fn get_starter(&self) -> Box<dyn BackendStarter> {
        return self.starter.clone();
    }
}
