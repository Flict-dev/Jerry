FROM rust:1.31

WORKDIR /usr/src/jerry
COPY . .
EXPOSE 7878
RUN cargo install --path .

CMD ["jerry"]