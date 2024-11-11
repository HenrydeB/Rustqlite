# Welcome to RustQLite
This is a lightweight database management system inspired by the structure of SQLite.
This project includes a simple SQL interpreter that is able to process a few basic versions of the following commands: 
CREATE, SELECT, INSERT, UPDATE, DROP and DELETE. 

## Database Structure
This project only allows for a single database, which is found in `data/database.rdb` for now (you may need to create your own `data` directory when you fork. This single database acts similarly to SQLite where all tables are found on one file. This database is organized as a BTreeMap, where the unique identifier of the table is the table name, and the table itself is stored as the value.

To view your current tables in your database, use the `schema` command.

## Running the Program
In it's current state, this program requires you to have Rust and Cargo installed on your machine. 
After cloning this repository, running `cargo run` in your terminal will open the basic RustQLite 
repl where you can immediately start writing commands. On a successful command, this will return an Ok() and the 
statement definition that you submitted. Otherwise, this will return an error that defines where the issue in your 
code is.

## Available Commands

> [!NOTE]
> ALL commands must end with a `;` terminator, otherwise this will be identified as an invalid statement.
> ALL commands are confied to a single line.
>
> A special note on WHERE clauses: when you list a series of conditions in a WHERE, this project only supports AND in the sense that if any row
> has a conditions that would match a value within any one of your WHERE clauses, this would be a valid row for the statement. This was done for the
> sake of simplicity an due to time constraints, but will be updated in the future.

### SELECT
A select statement can request all columns from the target table using an aserisk `*` or a collection of desired columns from the table by listing them:

```SQL
SELECT * FROM <table name>;

SELECT column1, column2 FROM <table>;

SELECT <column set> FROM <table> WHERE <conditions>
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
Possible data types you can pass in are `int`, `varchar`, and `bit`. Here, `int` can take any valid non-floating point number, `varchar` accepts a string of characters surrounded by `' '` single quotes, and a bit will accept the values `true` or `false`. You may wonder, why should `bit` accept written true or false instead of `1` or `0`? The `bit` datatype was written as such to mimic SQL server's syntax, though the implementation on the back end is used as true or false, so in an effort to make it more clear to everyone involved, we allow it to accept `true` or `false`.

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
### Other Commands
* `schema` will print out the available table names you have created
* `exit` will exit the program

## Interacting with the VM

When executing a command, the Rustqlite virtual machine will provide feedback to you to help you understand if a command was successful or not. Whether it is a SELECT statement, which will return the target table or inform you that the table does not exist, or any other "WRITE" actions to a database, the  virtual machine will respond with a success message or not.

## Architectural Overview
