FROM rust:1.67
LABEL authors="zaiquiri"

WORKDIR ./

COPY Cargo.toml ./
COPY Cargo.lock ./
RUN echo "fn main() {}" > dummy.rs
RUN sed -i 's#src/main.rs#dummy.rs#' Cargo.toml
RUN cargo build
RUN sed -i 's#dummy.rs#src/main.rs#' Cargo.toml

COPY . .
RUN cargo install --path .

EXPOSE 14293

ENTRYPOINT ["protohackers", "prime-time"]