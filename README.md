# Near-RSS

A self-hosted RSS reader implementing Google-Reader API.

The UI is a copy of the [stringer](https://github.com/swanson/stringer) project, plus a little more customizations.

![img](https://i.imgur.com/DDHwUTn.png)

## Features

1. A self-contained RSS reader, including
   * a server, which manages your subscriptions and your read status.
   * a user-friendly web-ui
2. Implements Google-Reader API. You can use your favorite RSS client to connect your instance.
3. A PWA which you can pin it in your mobile device without installing any app.

## Shortcuts

* **j/k**: up or down.
* **z**: mark all items above the current selected one as read.
* **v**: open the item in a new browser tab.
* **e**: expand the current selected item to see more details.

## Local test

```
# Initialize your Sqlite database.
sqlx database create
sqlx migrate run

APP_APPLICATION__EMAIL=your-email@email.com APP_APPLICATION__PASSWORD=1234 cargo run
```

It will start both the server and the web-ui at localhost:7555. You can use `email` and `password` to login.

## Deploy

Run the following commands to start your own instance. You can also update the `configuration/production.yaml` to change the config.

```
# Build
cargo build --release

# Initialize your Sqlite database.
sqlx database create
sqlx migrate run

APP_ENVIRONMENT=production APP_APPLICATION__EMAIL=your-email@email.com APP_APPLICATION__PASSWORD=1234 ./target/release/near-rss
```

## Import your subscriptions

```
# you may need to install the 'requests' module

export SUBS="your subscription OPML file"
export SERVER="your server hostname"
export EMAIL="your email"
export PASSWORD="your password"
python3 scripts/import.py $SUBS $SERVER $EMAIL $PASSWORD
```

## Customization

You can customize the static resources in the public directory.

## Some known issues

* The code needs to polish. My rust skills was rusty when I wrote this. I'm getting better (I hope), but this project hasn't.