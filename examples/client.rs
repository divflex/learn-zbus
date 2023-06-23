#[cfg(feature = "tokio")]
use tokio::net::UnixStream;
use zbus::{ConnectionBuilder as Builder, dbus_proxy, Result};


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

    let client = UnixStream::connect("greeter").await.unwrap();
    let conn = Builder::unix_stream(client).p2p().build().await?;
    let proxy = MyGreeterProxy::new(&conn).await?;
    let reply = proxy.say_hello("Maria").await?;

    println!("reply {}", reply);

    Ok(())
}