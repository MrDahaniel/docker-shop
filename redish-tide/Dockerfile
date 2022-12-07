FROM rust:1.64 

RUN git clone https://github.com/MrDahaniel/docker-shop.git

WORKDIR /docker-shop/redish-tide

RUN cargo install --path .

EXPOSE 8080

CMD ["redish-tide"]



