use std::{error::Error, future::pending};
use zbus::{ConnectionBuilder, dbus_interface};

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

/// use `dbus-launch`
/// 
/// use the result (DBUS_SESSION_BUS_ADDRESS/DBUS_SESSION_BUS_PID/DBUS_SESSION_BUS_WINDOWID) 
/// 
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let greeter = Greeter { count: 0 };
    let _conn = ConnectionBuilder::address("unix:abstract=/tmp/dbus-IP6WRN8s2E,guid=db0e4a25a334f6020e14ac3c6495ae5f")?
        .name("org.zbus.MyGreeter")?
        .serve_at("/org/zbus/MyGreeter", greeter)?
        .build()
        .await?;

    // Do other things or go to wait forever
    pending::<()>().await;

    Ok(())
}