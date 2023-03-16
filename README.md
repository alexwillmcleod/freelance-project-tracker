# Freelance

App for freelances to track the status of the their projects and share information with clients, as well as confirm and consent to changes to the project. Uses a rust based api with Axum, Mongo for the database, React-Vite for the front-end, and AWS and docker compose for hosting. Uses full authentication with JWT tokens and cookies for session management.

<img src="client/public/screenshot.png" />

## Environment Variables

MONGODB_URI - URI for MongoDB server
JWT_SECRET - Secret for JWT tokens to use for verification
PORT - Port for the server to be hosted on
COOKIE_DOMAIN - Domain for cookies

## Licence

MIT
