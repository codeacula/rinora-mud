# rinora-mud

The RinoraMUD

## Contributing

- Need to install PGSQL CLI Tools (and optionally pgAdmin 4)
- Add the `bin` and `lib` folder of the new PGSQL installation to your system PATH

## Migrations

We use [Diesel](https://diesel.rs/) to handle migrations. In order to create a migration, you need to install the Diesel CLI:

```bash
cargo install diesel_cli --no-default-features --features postgres
```
