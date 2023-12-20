# Configuration
```
# make a crates/server/.env file with the following

DATABASE_URL=postgresql://root:toor@localhost:5432/db

# run migrations with

sqlx migrate run

# reset db with

sqlx database reset

```
