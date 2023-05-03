FROM rust:1.64-buster

WORKDIR /usr/src/myapp
COPY . .
RUN apt update && apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt install -y build-essential \
    && apt install -y pkg-config  \
    && apt install -y librdkafka-dev \
    && apt install -y librdkafka++1 
    
RUN cargo install --path . && cargo clean

CMD ["tnt"]