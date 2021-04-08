# Rust/Actix/Postgres/Vue Proof of Concept

This POC is made for a later Open Source project, I will link it here once it is launched. Idea is to have a web server that can manipulate a Postgres database and well, do whatever web servers do. Actix was chosen for the server and Vue for the frontend.

The web server part is essentially a direct clone of [Harry Gill's](https://gill.net.in) very good tutorial. It can be found [Here.](https://gill.net.in/posts/auth-microservice-rust-actix-web1.0-diesel-complete-tutorial/) At the time of writing 2021, Rust is a relatively young language so bear in mind that newer and better techniques might be around.

### Prerequisites

- [Diesel_cli](http://diesel.rs/guides/getting-started/)
- Postgres
- Rust
- Node.js
- Email service API key, I used sparkpost account for this

### Deployment

1. Clone this repo
2. Create .env file to the root:
```
DATABASE_URL=postgres://user:password@server/dbname
SPARKPOST_API_KEY='xxxxx'
SENDING_EMAIL_ADDRESS='your.sendingaddress@yourdomain.com'
```
3. From the root, `mkdir public`
4. Go to App folder and run `npm install`
5. Still in the app folder, run `npm run dev`
6. From the root, setup diesel: `diesel migrations run` and then `diesel setup`
7. From the root, run `cargo run`
8. App SHOULD be up at 127.0.0.1:8086.
9. Navigate to 127.0.0.1:8086/app/login to see some functionality. Not everything works, this is very much a WIP at this state.
