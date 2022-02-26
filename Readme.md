# Running the bot
- [Install rust](https://www.rust-lang.org/tools/install)
- Create a mysql or mariadb database
- Create the database schema
  - For a blank db use the file `common/db.sql`
  - For the same state as before the bot was taken down, use `dump.sql` and replace the placeholder values in the settings table
- Set the `yuri_db` environment variable following this pattern: `mysal://user:password@host/database`
- Execute `cargo run`