use std::process::Command;
use crate::backend::Backend;

pub trait BackendStarter{
    fn start_backend(&self,port:u32);
}
pub struct CommandBackendStarter{
    file_path:String,

}
impl BackendStarter for CommandBackendStarter{
    fn start_backend(&self,port:u32){
        //调用系统cmd
        println!("start backend on port {}",port);
        let mut cmd = std::process::Command::new("cmd");
    }
}
impl CommandBackendStarter{
    fn new(path:String)->CommandBackendStarter{
        return CommandBackendStarter{
            file_path:path,
        }
    }
}
pub struct PythonBackendStarter{
    pub(crate) python_path:String,
    pub(crate) python_main_path:String,
}
impl BackendStarter for PythonBackendStarter{
    fn start_backend(self,port:u32){
        let mut cmd=Command::new(&self.python_path);
        cmd.arg(&self.python_main_path);
        cmd.arg("-p").arg(port.to_string());
        cmd.spawn().expect("Unable to start backend!");
    }
}
impl PythonBackendStarter{
    pub fn new(python_path:String,python_main_path:String)->PythonBackendStarter{
        return PythonBackendStarter{
            python_path,
            python_main_path,
        }
    }
}