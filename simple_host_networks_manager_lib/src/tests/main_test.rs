#[cfg(test)]
mod tests {


use default_net::interface;

use crate::*;


    #[tokio::test]
    async fn get_all_networking_phisical_interfaces_test() {

        let c=get_all_networking_phisical_interfaces().await;


        assert!(c.is_ok());
        let interfaces=c.unwrap();

        println!("{:?}",interfaces.len());


        
    }
    #[tokio::test]

    async fn get_wifi_networks_test() {

        let c=get_wifi_networks().await;


        assert!(c.is_ok());
        let interfaces=c.unwrap();

        println!("{:?}",interfaces.len());


        
    }
}
