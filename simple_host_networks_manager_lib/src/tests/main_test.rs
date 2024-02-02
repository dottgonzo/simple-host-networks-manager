#[cfg(test)]
mod tests {


use crate::*;


    #[tokio::test]
    async fn connection() {

        let c=connect().await;

        assert!(c.is_ok());

        
    }
}
