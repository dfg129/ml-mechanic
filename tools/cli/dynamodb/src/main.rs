use aws_config::meta::region::RegionProviderChain;
use aws_sdk_dynamodb::model::{
    AttributeDefinition, KeySchemaElement, KeyType, ProvisionedThroughput, ScalarAttributeType
};
use aws_sdk_dynamodb::{Client, Error, Region, PKG_VERSION};
use std::process;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    /// The region
    #[structopt(short, long)]
    region: Option<String>,

    /// The table name
    #[structopt(short, long)]
    table: String,

    /// The primary key
    #[structopt(short, long)]
    key: String,

    /// Activate verbose mode
    #[structopt(short, long)]
    verbose: bool,
}

async fn create_table(client: &Client, table: &str, key: &str) -> Result<(), Error> {
    let a_name: String = key.into();
    let table_name: String = table.into();

    let ad = AttributeDefinition::builder()
        .attribute_name(&a_name)
        .attribute_type(ScalarAttributeType::S)
        .build();

    let ks = KeySchemaElement::builder()
        .attribute_name(&a_name)
        .key_type(KeyType::Hash)
        .build();

    let pt = ProvisionedThroughput::builder()
        .read_capacity_units(10)
        .write_capacity_units(5)
        .build();

    match client
        .create_table()
        .table_name(table_name)
        .key_schema(ks)
        .attribute_definitions(ad)
        .provisioned_throughput(pt)
        .send()
        .await
    {
        Ok(_) => println!("Added table {} with key {}", table, key),
        Err(e) => {
            print!("Got an error creating table");
            print!("{}", e);
            process::exit(1);
        }
    };

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt::init();
    let Opt {
        table,
        key,
        region,
        verbose,
    } = Opt::from_args();

    let region_provider = RegionProviderChain::first_try(region.map(Region::new))
        .or_default_provider()
        .or_else(Region::new("us-east-1"));
    println!();

    onfig = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&shared_config);

    create_table(&client, &table, &key).await.unwrap();

    let req = client.list_tables().limit(10);
    let resp = req.send().await?;
    println!("Current Dynamodb table: {:?}", resp.table_names);
    Ok(())
}

