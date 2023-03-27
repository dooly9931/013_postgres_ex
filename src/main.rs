use postgres_ex::db::DbClient;

const URI: &str = "postgres://postgres:postgrespw@localhost:55001";

#[tokio::main]
async fn main() {
    // let mut client = DbClient::new(URI).await;

    // client.show_table_schema("current").await;

    // client.show_table_schema("updates").await;

    // client.insert_current("btc", "0.123456").await;

    // client.select_latest_current().await;

    // client.insert_current("eth", "123.456").await;

    // client
    //     .insert_update(1679563100000000, "btc", "-0.123")
    //     .await;

    // client.select_all_current().await;

    // client.select_all_update().await;

    // let target = client.read_target().await;

    // println!("{:?}", target);

    println!("{URI}/casanova");
}
