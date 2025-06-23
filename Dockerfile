# ---- Builder ----
FROM rust:slim-bookworm AS builder
WORKDIR /app

COPY Cargo.toml Cargo.lock ./

# Create dummy main.rs file
RUN mkdir src \
  && echo 'fn main() {}' > src/main.rs

# Build only dependencies
RUN cargo build --release --locked

# Copy all files and build binary
COPY . .
RUN touch src/main.rs \
  && cargo build --release --locked


# ---- Runtime ----
FROM gcr.io/distroless/cc-debian12:nonroot AS runtime
WORKDIR /app

COPY --from=builder /app/target/release/valsoray-dev ./
COPY --from=builder /app/assets ./assets

ENTRYPOINT [ "/app/valsoray-dev" ]
