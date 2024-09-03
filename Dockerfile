# Use the official Node.js image from the Docker Hub
FROM node:20.15.1-alpine3.20 as build
# FROM rust:1.67
FROM backpackapp/build:v0.30.1

# Set the working directory in the container
WORKDIR /

# Copy package.json and package-lock.json to the container
COPY package*.json ./

# Install application dependencies
RUN apt-get update && apt-get install
# Install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
# RUN curl "https://sh.rustup.rs" -sfo rustup.sh && \
#     sh rustup.sh -y && \
#     rustup component add rustfmt clippy
# Install Solana tools
# RUN sh -c "$(curl -sSfL https://release.solana.com/v1.18.18/install)"
# RUN sh -c "$(curl -sSfL https://release.solana.com/${SOLANA_CLI}/install)"
RUN yarn install

# RUN apt-get upgrade && apt-get install -y pkg-config build-essential libudev-dev

# Install Anchor 
# RUN cargo install --git https://github.com/coral-xyz/anchor --tag v0.30.1 anchor-cli --locked
# RUN cargo install --git https://github.com/coral-xyz/anchor --tag ${ANCHOR_CLI} anchor-cli --locked
# RUN cargo install --git https://github.com/coral-xyz/anchor --tag v0.30.1 anchor-cli --locked
# RUN avm install latest

RUN anchor build

# Copy the rest of the application source code to the container
COPY . .

# Expose the port your application will run on
EXPOSE 3000

# RUN chmod +x deployment-service
# Define the command to run your application
CMD ["anchor test"]