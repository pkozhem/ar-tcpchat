use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    sync::broadcast,
};
mod cfg;

static BROADCAST_CAP: usize = 10;

#[tokio::main]
async fn main() {
    let cfg = cfg::parse_envs().unwrap();
    let addr = cfg.get_addr();

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    let (sender, _reciever) = broadcast::channel::<String>(BROADCAST_CAP);

    loop {
        let (mut socket, _socket_addr) = listener.accept().await.unwrap();
        let sender = sender.clone();
        let mut reciever = sender.subscribe();

        tokio::spawn(async move {
            let (reader, mut writer) = socket.split();
            let mut reader = BufReader::new(reader);
            let mut str_buf = String::new();

            loop {
                tokio::select! {
                    reader_result = reader.read_line(&mut str_buf) => {
                        if reader_result.unwrap() == 0 {
                            break;
                        }
                        sender.send(str_buf.clone()).unwrap();
                        str_buf.clear();
                    }
                    reciever_result = reciever.recv() => {
                        let msg = reciever_result.unwrap();
                        writer.write_all(msg.as_bytes()).await.unwrap();
                    }
                };
            }
        });
    }
}
