use crate::backend_connector::backend::Backend;

pub struct BackendPool{
    backends:Vec<Backend>,
    current_index:i32,
}
impl BackendPool{
    pub fn new() -> BackendPool {
        return BackendPool{
            backends:Vec::new(),
            current_index:0,
        }
    }
    pub fn add_backend(&mut self, backend:Backend){
        self.backends.push(backend);
    }
    pub fn get_backend_at(& self, index:i32)->Option<Backend>{
        return if index < 0 || index >= self.backends.len() as i32 {
            None
        } else {
            Some(self.backends[index as usize].clone())
        }
    }
    pub fn start_listening(&mut self){

    }
}
#[cfg(test)]
mod tests {
    use crate::add;
    use crate::backend_connector::backend::Backend;

    #[test]
    fn it_works() {
        let backend=Backend::new("127.0.0.1".to_string(), 8080);
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
