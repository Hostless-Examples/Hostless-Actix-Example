# Use the Rust official image as the base image
FROM rust:latest AS builder

# Set the working directory inside the container
# WORKDIR /app
WORKDIR /usr/src/app

# Copy the dependency manifests
COPY . .

# # Build the application
RUN cargo build --release

# Install the application
# RUN cargo install --path .

# Start a new stage and use a lightweight image for the final runtime
FROM debian:12 as runner

# Set the working directory inside the container
WORKDIR /usr/src/app

# Copy the built binary from the builder stage to the final stage
# COPY --from=builder /usr/local/cargo/bin/hostless-actix-app /usr/local/bin/hostless-actix-app
COPY --from=builder /usr/src/app/target/release/hostless-actix-app .
COPY --from=builder /usr/src/app/templates ./templates

# Expose the port your application listens on
EXPOSE 8000

# Command to run the application
CMD ["./hostless-actix-app"]