# poc-rust-vue

## Deployment

Instructions very much subject to change and might already be outdated! Rust+Cargo and NPM are supposed to be installed to begin with and who knows how this will behave on Windows.

1. Clone this repo
2. Create .env file to the root with `DATABASE_URL=test.db`
3. If you haven't already, install Diesel as per instructions in the example below.
4. From the root, run `cargo run`
5. Go to App folder and run `npm install`
6. Still in the app folder, run `npm run dev`
7. Site SHOULD be up at 127.0.0.1:8084. It's a treefloof!
8. Navigate to 127.0.0.1:8084/app/login to see some functionality.

## Useful stuff

Actix/Diesel example: https://github.com/actix/examples/tree/master/database_interactions/diesel