#[cfg(test)]
mod tests;


pub async fn connect() -> anyhow::Result<()> {
    // check ping
    let internet_status = check_server_connection().await;
    if internet_status.is_ok() {
        Ok(())
    } else {
        let connection_status = check_connection().await;

        if connection_status.is_ok() {
            Ok(())
        } else {
            Err(anyhow::anyhow!("no internet connection"))
        }
    }
    // if ping fail, check if wifi is connected

}

pub async fn check_server_connection() -> anyhow::Result<()> {
    Ok(())
}

pub async fn check_connection() -> anyhow::Result<()> {
    // check ping
    // if ping fail, check if wifi is connected

    let pinged = rust_simple_ping::ping().await;

    match pinged {
        Ok(_) => Ok(()),
        Err(_) => Err(anyhow::anyhow!("ping failed")),
    }
}

