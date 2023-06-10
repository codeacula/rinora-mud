FROM rust:buster AS rinoramud
# Update default packages
RUN apt-get update

# Get Ubuntu packages
RUN apt-get install -y g++ pkg-config libx11-dev libasound2-dev libudev-dev

# Update new packages
RUN apt-get update

FROM rinoramud
COPY . .
RUN cargo build --release
CMD ["./target/release/rinora-mud"]