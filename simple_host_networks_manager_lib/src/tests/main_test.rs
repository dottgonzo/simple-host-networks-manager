#[cfg(test)]
mod tests {

    use crate::*;

    #[tokio::test]
    async fn get_all_networking_phisical_interfaces_test() {
        let c = get_all_networking_phisical_interfaces().await;

        assert!(c.is_ok());
        let interfaces = c.unwrap();

        println!("{:?}", interfaces.len());
    }
    #[tokio::test]

    async fn get_wifi_networks_test() {
        let c = get_wifi_networks().await;

        assert!(c.is_ok());
        let interfaces = c.unwrap();

        println!("{:?}", interfaces.len());
    }
    #[tokio::test]

    async fn get_routes_networks_test() {
        let r = get_routes().await;

        assert!(r.is_ok());

        println!("{:?}", r);
    }
}
