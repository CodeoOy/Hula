# poc-rust-vue

Started by cloning the backend code from here: https://github.com/actix/examples/tree/master/database_interactions/diesel

## Deployment

Instructions very much subject to change and might already be outdated!

1. Clone this repo
2. Create .env file to the root with `DATABASE_URL=test.db`
3. If you haven't already, install Diesel as per instructions above
4. From the root, run `cargo run`
5. Go to App folder and run `npm install`
6. Still in the app folder, run `npm run dev`
7. Site SHOULD be up at 127.0.0.1:8084. It's a treefloof!
8. Navigate to 127.0.0.1:8084/app/login to see some functionality.