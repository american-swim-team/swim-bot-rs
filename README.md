# swim> server
This is a swim> server written in Rust using the Warp web framework. It connects to a Postgres database and acts as a Discord bot.

## Configuration
The application requires a config.toml file to be present in the root directory. You can use the config-example.toml file as a template for your own configuration.

## Database
Currently needs a table called steamids, use the following command on your DB.
```
CREATE TABLE steamids (
    discordid BIGINT UNIQUE,
    steamid BIGINT
);
```

## Running the Application
To run the application, you can use the following command:
´cargo run´

This will start the server on http://localhost:3030.