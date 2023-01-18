FROM rust:latest
COPY ./ ./

# Environment Variables
ENV MONGODB_URI = 'mongodb+srv://root:bF2yyBFnBFzvTs6@cluster0.kp8qsde.mongodb.net/?retryWrites=true&w=majority'
ENV JWT_SECRET = '936DA01F9ABD4d9d80C702AF85C822A8jFfdsasfdsFKD&%*#)4e244324;jfja;fi3'

RUN cargo build --release
CMD ["cargo", "run", "--release"]
EXPOSE 3000