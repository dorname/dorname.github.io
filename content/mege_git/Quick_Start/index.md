---
title: Mega Quick Start Guide on Ubuntu 22.04
date: 2023-11-28
extra:
    image: ../git.png
taxonomies:
  tags:
    - rust
    - git
  authors:
    - liguangqiao  
---
# Quick Started for developing and testing on Ubuntu(22.04)

## Install Rust on your Ubuntu machine.

```
TODO 按照Rust
```

## Clone mega repository and build it.

```
git clone https://github.com/web3infra-foundation/mega.git
cd mega
cargo build
```

## Install PostgreSQL and init database.

1. install PostgreSQL

2. create database,then find the dump file in the SQL directory of the Mega repository and import it into the database.

3. Create user and grant privileges.

4. Install redis.

5. Config environment variables for local test. For local testing, Mega uses the .env file to configure the required parameters. However, before starting the project, you also need to configure the environment variables such as `DB_USERNAME`, `DB_SECRET`, and `DB_HOST`.

6. init the Mega

   ```
   cd mega
   cargo run init
   ```

7. Start the Mega server for tesing

   ```
   cargo run https
   ```

8. Test the `git push` and `git clone`

   ```
   cd mega
   git remote add local http://localhost:8000/projects/mega.git
   git push local main
   cd / && mkdir temp_git
   cd temp_git
   git clone http://localhost:8000/projects/mega.git
   ```
