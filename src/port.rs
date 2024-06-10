use std::net::TcpListener;

pub fn find_available_port() -> u16 {
    return std::net::TcpListener::bind("0.0.0.0:0")
        .unwrap()
        .local_addr()
        .unwrap()
        .port();
}

fn is_port_occupied(port: u16) -> bool {
    match TcpListener::bind(("0.0.0.0", port)) {
        Ok(_) => false, // 绑定成功，说明端口未被占用
        Err(ref e) => true, // 其他错误直接返回
    }
}
#[cfg(test)]
mod tests {
    use crate::add;
    use crate::port::find_available_port;


    #[test]
    fn it_works() {

        println!("{}",find_available_port())
    }
}
