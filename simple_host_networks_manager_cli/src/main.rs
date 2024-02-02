use simple_host_networks_manager_lib;


#[tokio::main]
async fn main(){
    println!("Hello, world!");
    _=simple_host_networks_manager_lib::connect().await;
}