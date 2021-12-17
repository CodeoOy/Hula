# Hula

Web app for managing and matching of candidates to projects based on skills.

## Techs

Vue is used on the front-end.

The web server (Rust + Actix) implementation is based on [Harry Gill's](https://gill.net.in) very good tutorial. It can be found [here.](https://gill.net.in/posts/auth-microservice-rust-actix-web1.0-diesel-complete-tutorial/) At the time of writing (2021), Rust is a relatively young language so bear in mind that newer and better techniques might be around.

## Prerequisites

- Postgres
- Rust
- [Diesel_cli](http://diesel.rs/guides/getting-started/)
    * `cargo install diesel_cli --no-default-features --features postgres`
- Node.js
- Sparkpost API key

## Deployment

1. Create a postgres database
2. Clone this repo
3. Create .env file to the root:
```
DATABASE_URL=postgres://user:password@server/dbname
SPARKPOST_API_KEY='xxxxx'
SENDING_EMAIL_ADDRESS='your.sendingaddress@yourdomain.com'
SERVER_URL='localhost:8086'
PUBLIC_URL='http://localhost:8086'
RUST_LOG='trace'
USER_UPLOAD_PATH='/'
PROJECT_EXPIRY_SECS=2592000
```
4. From the root, `mkdir public`
5. Go to App folder and run `npm install`
6. Because of reasons, Flashmessages won't install as expected. Run `npm i @smartweb/vue-flash-message@1.0.0-alpha.12`
7. Bootstrap needs Popper but doesn't include it. Still in app folder, run `npm i @popperjs/core`
8. Still in the app folder, run `npm run dev`
9. From the root, setup diesel: `diesel migration run` and then `diesel setup`
10. From the root, run `cargo run`
11. App SHOULD be up at localhost:8086.

## Customizing the User Interface

The User Interface is built with [Bootstrap](https://getbootstrap.com) and can be customized with variables. The built in theme is located in the `app/src/styles/hula` directory and site specific customizations should be in `app/src/styles/custom`. The built in theme is completely ignored when `custom` exists.

### Theme structure

```
app/src/styles/custom
├── _main.scss
├── _variables.scss
└── assets
    └── bg.jpg
```

**\_variables.scss**  
Define colors and other variables for Bootstrap. `_variables.scss` is prepended to the `main.scss` and it doesn't know about any Bootstrap variables. Any default Bootstrap variables (like `$primary: lighten($blue, 10%);`) need to be copied from Bootstrap's `_variables.scss`.

**\_main.scss**  
Override default Bootstrap and Hula styles. `_main.scss` is appended to the `main.scss` and all Bootstrap variables, mixins and functions can be used.

**assets directory**  
Custom assets, like background image. Files in `assets` are copied to `public/assets`.
