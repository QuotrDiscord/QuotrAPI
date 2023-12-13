# QuotrAPI
This is going to be the API behind the Discord bot of the same name and the dashboard website that goes along with it.

If you wish to self-host a Quotr bot, you will need to link it to your own instance of the Quotr API.

## Technologies
### [Actix](https://actix.rs/)
1. It can be deployed on [Shuttle](https://www.shuttle.rs/pricing) for free with options to move to an [Nginx](https://nginx.org) server if it becomes more advantageous in the future.
2. It has support for tests, which can be useful to ensure everything works as planned without having to use external API testing tools.
3. Support for Websockets which I might use for the live-stats graphs on the website.

### [SeaORM](https://www.sea-ql.org/SeaORM/)
1. The first reason I'm choosing this instead of [Diesel](https://diesel.rs/) is admittedly a "me problem". Because I couldn't get the Diesel CLI working with the MySQL client for the life of me. (Yes, I read the documentation and a plethora of threads on the subject)
    - Note: The MySQL client part is relevant specifically because Planetscale (The database provider I intend to use) relies on MySQL.
2. The second reason I'm using SeaORM is that it supports migrations, Async operations, sub-queries, and tests. Things that could be useful for features I have in mind.

### [Shuttle](https://www.shuttle.rs/)
1. This is a choice made with heavy consideration of the free tier. After all, I don't want to be bleeding money for a learning project, but also because it seems to be a convenient way to host the API. I'm putting this here since it requires some level of code integration. Should the bot make revenue at all, or even enough to sustain a standalone Nginx server, I will consider migrating to that.

### Some form of JWT library

## Set-up
