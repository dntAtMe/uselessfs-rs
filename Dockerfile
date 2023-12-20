FROM rust:1.74.1

WORKDIR /usr/workspace/app
COPY . .

RUN apt-get update && apt-get install -y fuse3 libfuse3-dev
RUN cargo install --path .

CMD ["app"]
