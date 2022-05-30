FROM ubuntu:22.04

RUN apt-get update -y
RUN apt-get install -y curl bash sudo build-essential

# Install rust & wasm32-unknown-unknown
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
RUN rustup target add wasm32-unknown-unknown

# Install Node v18
RUN curl -fsSL https://deb.nodesource.com/setup_18.x | sudo -E bash -
RUN apt-get install -y nodejs
RUN npm install -g near-cli

COPY . .

WORKDIR /contract


