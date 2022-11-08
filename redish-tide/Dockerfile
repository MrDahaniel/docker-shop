FROM rust:1.64 

WORKDIR /usr/src/redish_tide
COPY . .

RUN cargo install --path .

CMD ["redish-tide"]

