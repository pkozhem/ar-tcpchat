mod cfg;
mod tcp;

#[tokio::main]
async fn main() {
    tcp::handle_tcp().await;
}
