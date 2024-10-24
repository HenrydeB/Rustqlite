# Welcome to RustQLite
This is a lightweight database management system inspired by the structure of SQLite.
This project includes a simple SQL interpreter that is able to process a few basic commands,
such as SELECT, INSERT, UPDATE, and DELETE. 

> [!NOTE]
> This project is a work in progress, a further iteration will be done to optimize and condense the codebase.
>

## Database Structure
This project only allows for a single database, which is found in the database.rdb file that is created
in the root directory of this program when you first run it (this will later be moved to another location). 

## Running the Program
In it's current state, this program requires you to have Rust and Cargo installed on your machine. 
After cloning this repository, running `cargo run` in your terminal will open the basic RustQLite 
repl where you can immediately start writing commands. On a successful command, this will return an Ok() and the 
statement definition that you submitted. Otherwise, this will return an error that defines where the issue in your 
code is.

## Available Commands

> [!NOTE]
> ALL commands must end with a `;` terminator, otherwise this will be identified as an invalid statement.
> ALL commands are confied to a single line as of 10/21/2024.
>
> The SQL specific command keywords (SELECT, INSERT INTO, etc.), 

### SELECT
A select statement can request all columns from the target table using an aserisk `*` or a collection of desired columns from the table by listing them:

```SQL
SELECT * FROM <table name>;

SELECT column1, column2 FROM <table column>;
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

### CREATE TABLE
To create a new table, use the **CREATE TABLE** command, followed by a comma delimited list within parenthesis that defines the column name and the type.
```SQL
CREATE TABLE <table_name> (col1 datatype, col2 datatype);
```
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
