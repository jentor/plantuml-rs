FROM rust:bookworm AS builder

WORKDIR /app

RUN rustup target add wasm32-unknown-unknown
RUN cargo install --locked wasm-pack

COPY . .

# Собираем web-target пакет и кладем его сразу в playground/pkg
RUN cd crates/plantuml-wasm && wasm-pack build --target web --out-dir ../../playground/pkg --release

FROM nginxinc/nginx-unprivileged:1.29-alpine AS runtime

COPY docker/nginx.conf /etc/nginx/conf.d/default.conf
COPY --from=builder /app/playground/ /usr/share/nginx/html/

EXPOSE 8080
