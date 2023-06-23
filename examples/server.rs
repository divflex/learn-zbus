
use tokio::net::{ UnixListener};
use zbus::{ConnectionBuilder as Builder, Guid, dbus_interface, Result};


#[derive(Debug, Clone, Copy)]
struct Greeter {
    count: u64
}

#[dbus_interface(name = "org.zbus.MyGreeter1")]
impl Greeter {
    // Can be `async` as well.
    fn say_hello(&mut self, name: &str) -> String {
        self.count += 1;
        format!("Hello {}! I have been called {} times.", name, self.count)
    }
}



#[tokio::main]
async fn main() -> Result<()> {

    let greeter = Greeter { count: 0 };
    let guid = Guid::generate();

    let listener  = UnixListener::bind("greeter").unwrap();

    loop {
        match listener.accept().await {
            Ok((stream, _addr)) => {
                let _conn = Builder::unix_stream(stream)
                    .server(&guid)
                    .p2p()
                    .name("org.zbus.MyGreeter")?
                    .serve_at("/org/zbus/MyGreeter", greeter.clone())?
                    .build()
                    .await?;
                println!("created {:?}", _conn);
            },
            Err(e) => {
                println!("failed {}", e);
            }
        }
    }
}