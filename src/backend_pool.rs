use std::thread::sleep;
use std::time::Duration;
use crate::backend::Backend;
use crate::port::find_available_port;

struct BackendInformation {
    backend: Box<dyn Backend>,

}

pub struct BackendPool {
    backends: Vec<Box<dyn Backend>>,
    current_index: i32,
    check_duration: Duration,
}

impl BackendPool {
    pub fn new() -> BackendPool {
        return BackendPool {
            backends: Vec::new(),
            current_index: 0,
            check_duration: Duration::from_millis(500),
        };
    }
    pub fn add_backend(&mut self, backend: Box<dyn Backend>) {
        self.backends.push(backend);
    }
    pub fn get_backend_at(&self, index: i32) -> Option<&Box<dyn Backend>> {
        return if index < 0 || index >= self.backends.len() as i32 {
            Option::None
        } else {
            Option::Some(&self.backends[index as usize])
        };
    }
    /*pub fn get_backend_at(& self, index:i32)->Option<Box<dyn Backend>>{
        return if index < 0 || index >= self.backends.len() as i32 {
            None
        } else {
            Some(self.backends[index as usize].clone())
        }
    }*/

    pub fn start_listening(&mut self) {
        loop {
            println!("1111");
            for backend in self.backends.iter() {
                println!("checking backend {}", backend.get_default_port());
                if backend.is_random_port() {
                    let available_port = find_available_port();
                    println!("server is starting on {}", available_port);
                }
                backend.get_start_command(1);
            }
            sleep(self.check_duration);
        }
    }
}

#[cfg(test)]
mod tests_backend_pool {
    use std::time::Duration;
    use crate::add;
    use crate::backend::{Backend, PythonBackend};
    use crate::backend_pool::BackendPool;

    #[test]
    fn it_works() {
        println!("111");
        let python_backend = PythonBackend {
            python_path: "".to_string(),
            python_main_path: "".to_string(),
            random_port: true,
            default_port: 8080,

        };
        let mut pool = BackendPool::new();
        pool.check_duration = Duration::from_millis(1);
        pool.add_backend(Box::new(python_backend));
        pool.start_listening();
    }
}
