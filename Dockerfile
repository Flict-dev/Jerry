FROM rust

ARG workers address

ENV RUST_LOG="debug"

COPY Cargo.toml Cargo.toml
COPY /src/ /src/
COPY /templates/ /templates/

EXPOSE 7878

RUN cargo build --release
CMD ["target/release/jerry"] 