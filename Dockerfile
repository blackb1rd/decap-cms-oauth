# Stage 1: Build the application
FROM rust:trixie AS builder

# Create a new empty shell project
WORKDIR /usr/src/app
COPY . .

# Build the application
RUN cargo build --release

# Stage 2: Create the final image
FROM debian:trixie

# EXPOSE uses the runtime PORT environment variable. EXPOSE is metadata; runtime mapping
# still requires using -p when running the container.

# Copy the binary from the builder stage
COPY --from=builder /usr/src/app/target/release/decap_cms_oauth /usr/local/bin/decap_cms_oauth

# Default port for the application at runtime (can be overridden when starting the container)
ENV PORT=3005
ENV ADDRESS=127.0.0.1

# Expose the port the application runs on (value follows the PORT build arg/env)
EXPOSE ${PORT}

# Set the entrypoint
CMD ["decap_cms_oauth"]
