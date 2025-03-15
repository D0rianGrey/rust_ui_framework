FROM rust:1.70-slim as builder

WORKDIR /app

# Копируем манифест и зависимости отдельно, чтобы кэшировать слои
COPY Cargo.toml Cargo.lock ./

# Создаем фиктивный main.rs для сборки зависимостей
RUN mkdir -p src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

# Копируем исходный код
COPY . .

# Собираем приложение
RUN cargo build --release

# Используем более легкий образ для запуска
FROM debian:bullseye-slim

WORKDIR /app

# Устанавливаем необходимые зависимости
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    ca-certificates \
    libssl-dev \
    curl && \
    rm -rf /var/lib/apt/lists/*

# Копируем бинарный файл из предыдущего этапа
COPY --from=builder /app/target/release/rust_ui_framework /app/
COPY config /app/config

# Устанавливаем переменную среды для конфигурации
ENV CONFIG_PATH=/app/config/default.toml
ENV RUST_LOG=info

# Запускаем приложение
CMD ["/app/rust_ui_framework"]