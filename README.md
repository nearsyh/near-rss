# Near-RSS

A self-hosted RSS server implementing Google-Reader API.

## Local run

```
cargo run
```

It will start both the server and the web-ui at localhost:8000. You can use `email` and `password` to login.

## Run

First, you need to update the `endpoint` value defined in `public/javascripts/connector.js` to your own hostname.

Then run the following commands to start your own instance.

```
cargo build --release

EMAIL="your-email" PASSWORD="your-password" DB="databasefile" ENDPOINT="your-server-address" ./target/release/near-rss
```