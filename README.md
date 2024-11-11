```OCaml
______          _   _____ _     _ _            /\
| ___ \        | | |  _  | |   (_) |          ( /   @ @    ()
| |_/ /   _ ___| |_| | | | |    _| |_ ___      \  __| |__  /
|    / | | / __| __| | | | |   | | __/ _ \      -/   V   \-
| |\ \ |_| \__ \ |_\ \/' / |___| | ||  __/     /-|       |-\
\_| \_\__,_|___/\__|\_/\_\_____/_|\__\___|    / /-\     /-\ \
                                              / /-\     /-\ \
                                             /   /-`---'-\ \
                                                /         \
```
# Welcome to RustQLite
This is a lightweight database management system inspired by the structure of SQLite.
This project includes a simple SQL interpreter that is able to process a few basic versions of the following commands: 
CREATE, SELECT, INSERT, UPDATE, DROP and DELETE. 

## Database Structure
This project only allows for a single database, which is found in `data/database.rdb` for now (you may need to create your own `data` directory when you fork). This single database acts similarly to SQLite where all tables are found on one file. This database is organized as a BTreeMap, where the unique identifier of the table is the table name, and the table itself is stored as the value.

To view your current tables in your database, use the `schema` command.

## Running the Program
In it's current state, this program requires you to have Rust and Cargo installed on your machine. 
After cloning this repository, running `cargo run` in your terminal will open the basic RustQLite 
repl where you can immediately start writing commands. 

### Running for the First Time

If you are running this program for the first time, I can assume that you may have an empty `data/` directory at the program's root (if not you should create one). Because you have an empty directory, when RustQLite attempts to read a database, this would normally fail because it will not be able to find a valid DB. Instead, you will get a warning message stating `No database found... creating new DB instance`. Then, if the command you entered requires a table to already exist (so not a CREATE), you will receive an error. 

_I would suggest using a CREATE command to start._

After a valid command is entered, the program will either respond with a table defenition for SELECT statements, or a green success message for all other commands. If the command is unsuccessful, the program will return a red error message, which will allow you to try again with updated syntax.

```SQL
# After running cargo run and RUSTQLITE welcome message is shown you can start entering commands

> SELECT * FROM <table name>;

<returns table definition>

```

## Available Commands

> [!NOTE]
> ALL commands must end with a `;` terminator, otherwise this will be identified as an invalid statement.
> ALL commands are confied to a single line.
>
> A special note on WHERE clauses: when you list a series of conditions in a WHERE, this project only supports AND in the sense that if any row
> has a conditions that would match a value within any one of your WHERE clauses, this would be a valid row for the statement. This was done for the
> sake of simplicity an due to time constraints, but will be updated in the future.

### Non-SQL Commands
* `schema` will print out the names of available tables
* `exit` will exit the program

### SELECT
A select statement can request all columns from the target table using an aserisk `*` or a collection of desired columns from the table by listing them:

```SQL
SELECT * FROM <table name>;

SELECT column1, column2 FROM <table>;
```
You can also include conditions to filter your select statement. You can have multiple conditions separated by an `AND` clause.

```SQL
SELECT * FROM <table name> WHERE <column> = <desired value>;

SELECT * FROM <table_name> WHERE <column_1> = <desired value> AND <column_2> = <desired value_2>;
```

### INSERT INTO
An **INSERT INTO** statement targets a table to insert a new row into. There are two forms of this statement, one where you define the target columns and values, 
and another where you define just the values. If you decide to only insert values without target columns, it is expected that you  are inserting values for all columns.
```SQL
INSERT INTO <table_name> (col1, col2, col3) VALUES (val1, val2, val3);
# or
INSERT INTO <table_name> VALUES (val1, val2, val3);
```

Note that if you don't define a value for the ID field when you are inserting, the ID will be generated for you.
You may also wish to not define a value for a particular column. You may do this if you would like, however no datatypes in this project are nullable, so the missed fields will be filled with their default values (numbers will be 0, varchar will be an empty string, and a bit will be false by default);

### CREATE TABLE
To create a new table, use the **CREATE TABLE** command, followed by a comma delimited list within parenthesis that defines the column name and the type.
```SQL
CREATE TABLE <table_name> (col1 datatype, col2 datatype);
```
Note that an ID field MUST be the first field that gets added, otherwise and ID column will be added for you. Subsequent ID columns must be IDs that reference a separate table.

#### DataTypes
Possible data types you can pass in are `int`, `varchar`, and `bit`. Here, `int` can take any valid non-floating point number, `varchar` accepts a string of characters surrounded by `' '` single quotes, and a bit will accept the values `true` or `false`. You may wonder, why should `bit` accept written true or false instead of `1` or `0`? The `bit` datatype was written as such to mimic SQL server's syntax, though the implementation on the back end is used as true or false. For the sake of simplicity in this project, I thought using `true` and `false` for this would work as a way to differenciate between this datatype and `int`, seeing as when we save to `database.rdb` it will be encoded the same way anyway.

### DROP TABLE
If you would like to drop a table you have already created, then the command is simply
```SQL
DROP TABLE <table_name>;
```

### DELETE FROM
To remove row(s) from a table, you must define conditons to filter which row(s) you would like to remove.
```SQL
DELETE FROM <table_name> WHERE col1 = val1 ...;
```
### UPDATE
An update statement will update a field or a set of fields in a row of a table. This requires a set of conditions to be defined on the statement.
> [!NOTE]
> This MUST include a *WHERE* clause, updating multiple rows at once is currently not supported.

```SQL
UPDATE <table_name> SET col1 = <desired_val> WHERE col2 = <curr_val>;
```

## Interacting with the VM

When executing a command, the Rustqlite virtual machine will provide feedback to you to help you understand if a command was successful or not. Whether it is a SELECT statement, which will return the target table or inform you that the table does not exist, or any other "WRITE" actions to a database, the  virtual machine will respond with a success message or not.

## Architectural Overview

The general overview of the architecture for this program is as follows

![alt text](https://github.com/HenrydeB/Rustqlite/blob/main/diagrams/arch_overview.drawio.png)

1. User first starts up the program
2. User inputs the command to be processed by the interpreter
3. Scanner processes the command into tokens and sends to parser
4. Parser processes tokens into relevant Stmt struct for command
5. Virtual Machine receives statement and processes it
6. Depending on command type, we will read or write to database
7. If error, return String converted into ColoredString colored red for error message, if success message is returned and colored green

One of the objectives of this project was to  avoid panics as much as possible. This project relies on error propagation from any one function to the entrypoint, which would then be colored for user experience purposes.

### Structs and Enums
![alt text](https://github.com/HenrydeB/Rustqlite/blob/main/diagrams/structs.drawio.png)

### Interpreter
#### enum TokenType
This enumerable contains different keywords that the scanner is looking out for when defining the token type. We are displaying the category of token types available in the image above, if you would like to see a more concise list of the token types please refer to `src/interpreter/tokens.rs`

#### enum Literal
This enumerable contains a series of tuple structs that represent the values of the fields to be saved within a table in our database. This acts like an Option type and includes a None for those tokens who do not need a value saved for the Literal field.

#### Token
This struct contains three fields: the TokenType, the lexeme, and the literal. As stated above, the literal is an Option as not all Tokens will need this field populated, which is why we included the Option type.

### Virtual Machine
#### Table
The table struct is the foundation of this project. Containg a name field along with a vector of Columns, a row field organized as a BTreeMap with the row ID being the key and the Row instance itself as the values, and finally a "schema" HashMap which contains Column Name as the key and the Datatype of the column as the value. This allows us to verify the input and update requests have the required data types before we commit them to our tables.

#### Column
This is a simple struct, only storing the name and the data type of the column.

#### Row
This contains a 'values' HashMap, which maps the column name to the literal to be set at that column "cell". 

#### Database
This is what organizes our `database.rdb` file. Set up as a BTreeMap that maps the table name and the table itself. 

#### Virtual Machine
The VM struct only contains the command Stmt that was passed in when the struct is created.

### enum Stmt

![alt text](https://github.com/HenrydeB/Rustqlite/blob/main/diagrams/statements.drawio.png)

As you can see, there are several kinds of statement structs within this enumerable. Because the objective of this project was to get the basic functionality of a database system to work, the statements were set up with a rather strict structure so the program can explicitly expect a certain series of values from the scanner and parser, though there are similarities in structure between some of the statements (where conditions, target values and columns, etc.). This would be a primary target for refactoring if this project were to continue.

### Crates Used
I made an effort to avoid using code outside of the standard library as much as I could, however there are a few that I used for stylistic purposes and a couple to help with serialization and binary encoding (for reading/writing the database). 

#### Style
After spending a lot of time attempting to create a good table structure to print my objects myself, out of frustration I began wondering if I  could find an existing crate that did this service. Fortunately, I did. Text-tables is a fantastic crate that takes in a Vec<Vec<String>> structure, and outputs it in tabular format.

I also used Colored to help with coloring the outputs of the success, error, and schema texts, for an improved user experience.

#### Serialization
While researching serialization/deserialization for this project, the top package that came up  for this was Serde. This happens to be one of the top crates used by Rustaceans, so I thought it would be a good fit. All structs that are a part of the data that gets set to our `database.rdb` file. Once the required attributes are set on those structs, we use the crate Bincode, which implements Serde, for binary serialization, which I thought was the best implementation of serialization for this project. 

Again, here we use `serde(serialize, deserialize)` to help different structs to be serialized, then we use the crate `bincode` that interfaces with `serde` to do the actual serialization of our data into our database file.

## Next Steps
There are a number of changes & features I would like to implement if I were to have more time. The first change I would implement is the parser. As stated before, the objective of this project was to attempt to create a simple database system. Because of this, I did not implement a recursive decent parser because I did not want to run out of time at the end. This would be the first thing I would change. I believe that implementing a recursive decent parser may provide me with the opportunity to alter the datatypes used within the structs for Tokens and Statements, hopefully minimizing the clone operations needed to get the tables printed/written.

The next step would be to investigate concurrency in this project. Though SQLite is frequently used as local storage for embeddes systems, I believe I could use this to create a separate database server that can be interacted with via an API. I think exposing this with an API via a library would be an interesting expansion on this project, with the goal of interacting with it via a web application. This would provide me with the opportunity to experiment with communicating with external applications and with concurrency, beginning to implement the ACID properties found in most databases.
