FROM rust:1.64 AS builder

WORKDIR /usr/src/myapp
COPY . .
RUN git clone https://github.com/Heiaha/asyncLio-bot.git bot
RUN cargo build --release

FROM python:3.10-buster

WORKDIR /usr/src
COPY --from=builder /usr/src/myapp/bot .
COPY --from=builder /usr/src/myapp/config.yml .
COPY --from=builder /usr/src/myapp/target/release/cui ./engine
RUN pip install -r requirements.txt

CMD ["python", "main.py", "-v"]