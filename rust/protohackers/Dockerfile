FROM rust:1.67
LABEL authors="zaiquiri"

WORKDIR ./

COPY Cargo.toml ./
COPY Cargo.lock ./
RUN echo "fn main() {}" > dummy.rs
RUN echo "fn main() {}" > dummy_test.rs
RUN sed -i 's#src/main.rs#dummy.rs#' Cargo.toml
RUN sed -i 's#src/test_runner.rs#dummy_test.rs#' Cargo.toml
RUN cargo build
RUN sed -i 's#dummy.rs#src/main.rs#' Cargo.toml
RUN sed -i 's#dummy_test.rs#src/test_runner#' Cargo.toml

COPY . .
RUN cargo install --path .

EXPOSE 8000

ENTRYPOINT ["protohackers", "udpdb", "0.0.0.0:8000"]