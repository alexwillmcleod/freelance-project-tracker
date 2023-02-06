# Freelance

App for freelances to track the status of the their projects and share information with clients, as well as confirm and consent to changes to the project. Uses a rust based api with Axum, as well as MongoDB for the database and Amazon S3 for storage.

## API Routes

/ GET. Returns in plain text "Hello, Anonymous User!" if you are not logged in with an auth token. "Hello, {firstName} {lastName}! Your ID is {id}" if you are logged in.

## Docker

Dockerfile is available for the api.

## Environment Variables

MONGODB_URI - URI for MongoDB server
JWT_SECRET - Secret for JWT tokens to use for verification
PORT - Port for the server to be hosted on
COOKIE_DOMAIN - Domain for cookies

## Licence

MIT
