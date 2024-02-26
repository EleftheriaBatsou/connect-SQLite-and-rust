### Connect Qust and SQLite

Overall, this Rust program connects to an SQLite database, creates the necessary tables and schema, inserts data into the database, and prints the result of the SQL query execution. It demonstrates how to interact with an SQLite database using the sqlx crate.

**1. Imports and Definitions:**
- The program imports necessary modules and traits from the std and sqlx crates.
- It defines an asynchronous function ```create_schema``` that takes a database URL (```db_url```) as input and returns a Result containing a ```SqliteQueryResult``` or a ```sqlx::Error```.
- It defines the main function marked with the ```async_std::main``` attribute.

**2.Database Schema Creation (create_schema function):**
- Inside the create_schema function, a connection pool (pool) to the SQLite database is established using SqlitePool::connect.
- The SQL query (```qry```) is defined to create two tables: ```settings``` and ```project```, along with their respective columns and constraints.
- The SQL query is executed using ```sqlx::query(&qry).execute(&pool).await```, and the result is stored in result.
- The connection pool is closed using ```pool.close().await```.

**3.Main Function Execution:**
- In the main function, a database URL (```db_url```) is specified, pointing to the SQLite database file ```sqlite.db```.
- It checks if the database file exists using Sqlite::database_exists, and if not, it creates the database file using ```Sqlite::create_database```.
- If the database doesn't exist or is created successfully, it calls the create_schema function to create the database schema.
- A connection pool (```instances```) to the database is established using ```SqlitePool::connect```.
- An SQL query (```qry```) is defined to insert a value into the settings table.
- The SQL query is executed using ```sqlx::query(&qry).bind("testing").execute(&instances).await```, and the result is stored in ```result```.
- The connection pool is closed using instances.close().await.

**4.Printing Result:**
The result of the SQL query execution is printed to the console using ```println!("{:?}", result)```.
