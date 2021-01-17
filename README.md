Some explorations around DDD and Rust for personal purpose and obviously not aimed for production.

Some todos:

- Authentication
- Better DI
- Better db pool
- Events
- Reporting
- ...

Create the `.env` file

```sh
cp .env.template .env
```

... and set the database url env. variable (user needs to have `CREATEDB` privilege)

Install the sqlx-cli

```sh
cargo install sqlx-cli
```

In the `infrastructure` directory launch the command:

```sh
sqlx database setup
```

Create a migration:

```
sqlx migrate add migration-name
```

Run pending migrations:

```sh
sqlx migrate run
```
