use scylladb_rs::ScyllaClient;
use scylladb_rs::query::query::*;
use scylladb_rs::query::query_builder::*;

#[tokio::test]
async fn integration_test()  {

    let query = QueryBuilder::new(Operations::Select, "employees")
    .select(&["id", "name", "department"])
    .where_condition(&logical_condition(
        &condition("age", ComparisonOperators::Gt, "25"),
        LogicalOperators::And,
        &logical_condition(
            &condition("department", ComparisonOperators::Eq, "'Engineering'"),
            LogicalOperators::Or,
            &condition("department", ComparisonOperators::Eq, "'Sales'"),
        ),
    ))
    .order_by("name", OrderDirection::Asc)
    .clause("LIMIT 10")
    .build();

    println!(" {}\n", query);

    let insert_query = QueryBuilder::new(Operations::Insert, "users")
        .select(&["id", "name", "age"])
        .insert_option(InsertOptions::UsingTimestamp(1627846723))
        .insert_option(InsertOptions::UsingTTL(3600))
        .build();

    println!(" {}\n", insert_query);

    let update_query = QueryBuilder::new(Operations::Update, "employees")
    .select(&["name", "salary"])
    .where_condition(&condition("id", ComparisonOperators::Eq, "123"))
    .where_condition(&logical_condition(
        &condition("department", ComparisonOperators::Eq, "'Engineering'"),
        LogicalOperators::And,
        &condition("salary", ComparisonOperators::Lt, "100000"),
    ))
    .clause("ALLOW FILTERING")
    .build();

    println!(" {}\n", update_query);

    let delete_query = QueryBuilder::new(Operations::Delete, "employees")
    .where_condition(&condition("id", ComparisonOperators::Eq, "123"))
    .where_condition(&logical_condition(
        &condition("department", ComparisonOperators::Eq, "'Engineering'"),
        LogicalOperators::And,
        &condition("age", ComparisonOperators::Gt, "60"),
    ))
    .build();

    println!(" {}\n", delete_query);

    let insert_query = QueryBuilder::new(Operations::Insert, "orders")
    .select(&["order_id", "customer_id", "amount", "order_date"])
    .insert_option(InsertOptions::UsingTimestamp(1627846723))
    .insert_option(InsertOptions::UsingTTL(86400))  // 1 day in seconds
    .build();

    println!(" {}\n", insert_query);

    let like_query = QueryBuilder::new(Operations::Select, "customers")
    .select(&["id", "name", "email"])
    .where_condition(&pattern_condition("email", PatternMatchingOperators::Like, "'%@example.com'"))
    .order_by("name", OrderDirection::Asc)
    .build();

    println!(" {}\n", like_query);

    let null_query = QueryBuilder::new(Operations::Select, "contacts")
    .select(&["id", "name", "phone"])
    .where_condition(&null_condition("phone", NullOperators::IsNull))
    .order_by("name", OrderDirection::Asc)
    .build();

    println!(" {}\n", null_query);


}