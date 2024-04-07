use rusoto_core::Region;
use rusoto_dynamodb::{
    DynamoDb, DynamoDbClient, CreateTableInput, AttributeDefinition, KeySchemaElement,
    ProvisionedThroughput, ListTablesInput, PutItemInput, AttributeValue,
    GetItemInput, DeleteItemInput,
};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = DynamoDbClient::new(Region::UsEast1); //resion aws 설정 

    let table_name = "SimpleTable";
    let list_tables_input = ListTablesInput::default();
    let list_tables_output = client.list_tables(list_tables_input).await?;

    if let Some(table_names) = list_tables_output.table_names {
        if table_names.contains(&table_name.to_string()) {
            println!("Table 'SimpleTable' already exists!");
        } else {
            let create_table_input = CreateTableInput {
                attribute_definitions: vec![
                    AttributeDefinition {
                        attribute_name: "userId".to_string(),
                        attribute_type: "S".to_string(),
                    },
                ],
                key_schema: vec![
                    KeySchemaElement {
                        attribute_name: "userId".to_string(),
                        key_type: "HASH".to_string(),
                    },
                ],
                provisioned_throughput: Some(ProvisionedThroughput {
                    read_capacity_units: 5,
                    write_capacity_units: 5,
                }),
                table_name: table_name.to_string(),
                ..Default::default()
            };

            client.create_table(create_table_input).await?;
            println!("Table 'SimpleTable' created successfully!");
        }
    }

    add(&client).await?;
    get(&client).await?;
    del(&client).await?;

    Ok(())
}

async fn add(client: &DynamoDbClient) -> Result<(), Box<dyn Error>> {
    let mut item: std::collections::HashMap<String, AttributeValue> = std::collections::HashMap::new();
    item.insert("userId".to_string(), AttributeValue {
        s: Some("user1".to_string()),
        ..Default::default()
    });
    item.insert("name".to_string(), AttributeValue {
        s: Some("John Doe".to_string()),
        ..Default::default()
    });
    item.insert("isInfected".to_string(), AttributeValue {
        bool: Some(false),
        ..Default::default()
    });

    let put_item_input = PutItemInput {
        table_name: "SimpleTable".to_string(),
        item,
        ..Default::default()
    };

    client.put_item(put_item_input).await?;
    println!("Item added successfully!");

    Ok(())
}

async fn get(client: &DynamoDbClient) -> Result<(), Box<dyn Error>> {
    let mut key: std::collections::HashMap<String, AttributeValue> = std::collections::HashMap::new();
    key.insert("userId".to_string(), AttributeValue {
        s: Some("user1".to_string()),
        ..Default::default()
    });

    let get_item_input = GetItemInput {
        table_name: "SimpleTable".to_string(),
        key,
        ..Default::default()
    };

    let output = client.get_item(get_item_input).await?;
    if let Some(item) = output.item {
        println!("Item retrieved successfully:");
        print_item(&item);
    } else {
        println!("Item not found!");
    }

    Ok(())
}

async fn del(client: &DynamoDbClient) -> Result<(), Box<dyn Error>> {
    println!("Enter the user id to delete:");
    let mut user_id = String::new();
    std::io::stdin().read_line(&mut user_id)?;

    let mut key = std::collections::HashMap::new();
    key.insert("userId".to_string(), AttributeValue {
        s: Some(user_id.trim().to_string()),
        ..Default::default()
    });

    let delete_item_input = DeleteItemInput {
        table_name: "SimpleTable".to_string(),
        key,
        ..Default::default()
    };

    client.delete_item(delete_item_input).await?;
    println!("Item deleted successfully!");

    Ok(())
}

fn print_item(item: &std::collections::HashMap<String, AttributeValue>) {
    for (key, value) in item {
        print!("{}: ", key);
        match value {
            AttributeValue { s: Some(val), .. } => println!("{}", val),
            AttributeValue { bool: Some(val), .. } => println!("{}", val),
            _ => println!(""),
        }
    }
}
