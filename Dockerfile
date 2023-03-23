FROM ubuntu:latest

# 必要なパッケージのインストール
RUN apt-get update && apt-get install -y curl build-essential gcc make git binutils libc6-dev

# Rustのインストール
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# デフォルトのシェルをbashに設定
SHELL ["/bin/bash", "-c"]

# ホストマシン上のファイルをコンテナ内にコピー
WORKDIR /app
COPY /src .

# コンパイルして実行
# RUN rustc main.rs
# CMD ["./main"]

