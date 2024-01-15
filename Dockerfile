FROM rustlang/rust:nightly-alpine as builder
RUN apk update && \
apk add --no-cache bash curl npm libc-dev binaryen 
RUN npm install -g sass
RUN curl --proto '=https' --tlsv1.2 -LsSf https://github.com/leptos-rs/cargo-leptos/releases/download/0.2.5/cargo-leptos-installer.sh | sh
RUN rustup target add wasm32-unknown-unknown

WORKDIR /usr/work
COPY . .

RUN cargo leptos build --release -vv


FROM rustlang/rust:nightly-alpine as runner

WORKDIR /usr/app

COPY --from=builder /usr/work/target/release/team-availibility-coordinator /usr/app/
COPY --from=builder /usr/work/target/site /usr/app/site
COPY --from=builder /usr/work/Cargo.toml /usr/app/

EXPOSE 3000
ENV LEPTOS_SITE_ADDR="0.0.0.0:3000"
ENV LEPTOS_SITE_ROOT=./site

CMD ["/usr/app/team-availibility-coordinator"]