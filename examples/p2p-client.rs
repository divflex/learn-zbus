#[cfg(feature = "tokio")]
use tokio::net::UnixStream;
use zbus::{ConnectionBuilder as Builder, Guid, dbus_interface, dbus_proxy, Result};


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


#[dbus_proxy(
    interface = "org.zbus.MyGreeter1",
    default_service = "org.zbus.MyGreeter",
    default_path = "/org/zbus/MyGreeter"
)]
trait MyGreeter {
    async fn say_hello(&self, name: &str) -> Result<String>;
}


#[tokio::main]
async fn main() -> Result<()> {

    let greeter = Greeter { count: 0 };


    let guid = Guid::generate();
    let (p0, p1) = UnixStream::pair().unwrap();
    let (client_conn, _server_conn) = futures_util::try_join!(
        // Client
        Builder::unix_stream(p0).p2p().build(),
        // Server
        Builder::unix_stream(p1).server(&guid).p2p().name("org.zbus.MyGreeter")?.serve_at("/org/zbus/MyGreeter", greeter)?.build(),
    )?;

    let proxy = MyGreeterProxy::new(&client_conn).await?;
    let reply = proxy.say_hello("Maria").await?;

    println!("reply {}", reply);

    Ok(())
}