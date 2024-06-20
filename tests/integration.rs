use scylladb_rs::ScyllaClient;

#[tokio::test]
async fn integration_test()  {

    let client = ScyllaClient::new(
        vec!["127.0.0.1:9042"]).await.unwrap();

    let keyspace_dropping_result = client.drop_keyspace(
        "test"
    ).await.unwrap();

    println!("Keyspace dropping result: {:?}", keyspace_dropping_result);

    let keyspace_creating_result = client.create_keyspace_simple(
        "test",
        1
    ).await.unwrap();

    println!("Keyspace creating result: {:?}", keyspace_creating_result);

    let table_creating_result = client.create_table(
        "test",
        "example_table",
        &["id","name"],
        &[("id", "int"), ("name", "text")],
        Some(&[("name", "ASC")]),
        Some(3600)
    ).await.unwrap();

    println!("Table creating result: {:?}", table_creating_result);


}