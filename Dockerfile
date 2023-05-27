# Prepare the base image
FROM lukemathwalker/cargo-chef:latest-rust-1.69.0 as chef
WORKDIR /app
RUN apt update && apt install lld clang -y # lld needed as we selected alternative linker


# Compute the lock-file for the project
FROM chef as planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json


# Build dependencies, then build project
FROM chef as builder
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies from the lock-file
RUN cargo chef cook --release --recipe-path=recipe.json

COPY . .
ENV SQLX_OFFLINE true
# Actually compile our project
RUN cargo build --release


# Setup the runtime
FROM debian:bullseye-slim as runtime
WORKDIR /app
# Install OpenSSL, it is dynamically linked by some deps
# Install ca-certificates, needed to establish HTTPS connections
RUN apt-get update -y \
  && apt-get install -y --no-install-recommends openssl ca-certificates \
  # Clean up
  && apt-get autoremove -y \
  && apt-get clean -y \
  && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/zero2prod zero2prod
COPY configuration configuration
ENV APP_ENVIRONMENT production
ENTRYPOINT ["./target/release/zero2prod"]
