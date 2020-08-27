use redis;

fn connect() -> redis::Client {
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut con = client.get_async_connection()?;

    con
}
