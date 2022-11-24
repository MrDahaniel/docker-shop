FROM rust:1.64 

WORKDIR /redish_tide
COPY . .

RUN cargo install --path .

CMD ["redish-tide"]

