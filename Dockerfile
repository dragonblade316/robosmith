FROM rust:1.90

WORKDIR /usr/src/robosmith
COPY . .

RUN apt-get update -y
RUN apt-get install -y libopus0 cmake
RUN cargo install --path .

CMD ["robosmith"]

