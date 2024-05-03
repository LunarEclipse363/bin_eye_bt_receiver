use bluer::{
    agent::Agent,
    rfcomm::{Profile, Role},
};
use clap::Parser;
use enigo::{Enigo, Keyboard, Settings};
use futures::StreamExt;
use std::str;
use tokio::io::AsyncReadExt;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// UUID of scaner
    #[arg(short, long, default_value = "8a8478c9-2ca8-404b-a0de-101f34ab71ae")]
    uuid: String,
    /// Keyboard output
    #[arg(short, long, default_value_t = false)]
    keyboard: bool,
}

#[tokio::main]
async fn main() -> bluer::Result<()> {
    let args = Args::parse();
    let my_uuid: uuid::Uuid = bluer::Uuid::parse_str(&args.uuid).unwrap();
    println!("{}", my_uuid.urn());
    let mut enigo = Enigo::new(&Settings::default()).unwrap();

    let session = bluer::Session::new().await?;
    let adapter = session.default_adapter().await?;
    adapter.set_powered(true).await?;
    adapter.set_discoverable(true).await?;
    adapter.set_discoverable_timeout(0).await?;
    adapter.set_pairable(false).await?;
    let agent = Agent::default();
    let _agent_hndl = session.register_agent(agent).await?;
    let profile: Profile = Profile {
        uuid: my_uuid,
        name: Some("Binary Eye Bluetooth receiver".to_string()),
        channel: Some(0),
        role: Some(Role::Server),
        require_authentication: Some(false),
        require_authorization: Some(false),
        auto_connect: Some(true),
        ..Default::default()
    };
    let mut hndl = session.register_profile(profile).await?;
    loop {
        println!("Waiting for connection on RFCOMM channel 0?");
        let req = hndl.next().await.expect("received no connect request");

        eprintln!("Accepted connection from {}", req.device());
        let mut stream = req.accept()?;
        loop {
            let buf_size = 1024;
            let mut buf = vec![0; buf_size as _];

            let n = match stream.read(&mut buf).await {
                Ok(0) => {
                    println!("Stream ended");
                    break;
                }
                Ok(n) => n,
                Err(err) => {
                    println!("Read failed: {}", &err);
                    break;
                }
            };
            let buf = &buf[..n];

            println!("Echoing {} bytes", buf.len());
            /*         if let Err(err) = stream.write_all(buf).await {
                println!("Write failed: {}", &err);
                continue;
            } */
            let s = match str::from_utf8(buf) {
                Ok(v) => v,
                Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
            };

            println!("result: {}", s);
            if args.keyboard {
                let _ = enigo.text(s);
                let _ = enigo.key(enigo::Key::Return, enigo::Direction::Press);
                let _ = enigo.key(enigo::Key::Return, enigo::Direction::Release);
            }
            //enigo.text("\r");
        }
    }
    //Ok(())
}
