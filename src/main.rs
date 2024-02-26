use std::result::Result;
use sqlx::{sqlite::SqliteQueryResult, Sqlite, SqlitePool, migrate::MigrateDatabase};

async fn create_schema(db_url:&str) -> Result<SqliteQueryResult, sqlx::Error> { // pass in the db_url, it expexts a SqliteQueryResult
    let pool = SqlitePool::connect(&db_url).await?; // and create a pool to connect to the db_url

    // create the qry
    let qry = 
    "PRAGMA foreign_keys = ON;
    CREATE TABLE IF NOT EXISTS settings
    (
        settings_id     INTEGER PRIMARY KEY NOT NULL,
        description     TEXT                NOT NULL,
        created_on      DATETIME DEFAULT    (datetime('now', 'localtime')),
        updated_on      DATETIME DEFAULT    (datetime('now', 'localtime')),
        done            BOOLEAN             NOT NULL DEFAULT 0
    );
    CREATE TABLE IF NOT EXISTS project
    (
        project_id      INTEGER PRIMARY KEY AUTOINCREMENT,
        product_name    TEXT,
        created_on      DATETIME DEFAULT    (datetime('now', 'localtime')),
        updated_on      DATETIME DEFAULT    (datetime('now', 'localtime')),
        img_directory   TEXT     NOT NULL,
        out_directory   TEXT     NOT NULL,
        status          TEXT     NOT NULL,
        settings_id     INTEGER  NOT NULL DEFAULT 1,
        FOREIGN KEY (settings_id) REFERENCES settings (settings_id) ON UPDATE SET NULL ON DELETE SET NULL
    );";
    let result = sqlx::query(&qry).execute(&pool).await; // run the qry
    pool.close().await; // close the pool
    return result;
}

#[async_std::main]
async fn main() {
    let db_url = String::from("sqlite://sqlite.db"); // it will create a file in the root of our folder, my database is names 'sqlite.db'
    if !Sqlite::database_exists(&db_url).await.unwrap_or(false){  // if the database doesn't exist, 
        Sqlite::create_database(&db_url).await.unwrap(); // create it
        match create_schema(&db_url).await {  // if it exists then we call the create_schema
            //if everything goes well then print the OK... else print the 'panic'
            Ok(_) => println!("database created succesfully"),
            Err(e) => panic!("{}", e)
        }
    }
    let instances = SqlitePool::connect(&db_url).await.unwrap(); // connect to the database url (db_url)
    //create a table, name it settings, and settings have a field called descriptions and we'll enter some values
    let qry = "INSERT INTO settings (description) VALUES($1)";
    // now let's run this query
    let result = sqlx::query(&qry).bind("testing").execute(&instances).await;

    instances.close().await;
    println!("{:?}", result);
}

// to see the db, open the file at https://sqliteviewer.app/ or any other sqlite viewer