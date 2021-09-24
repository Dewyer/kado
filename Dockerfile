FROM rust:1.53.0
RUN cargo install diesel_cli
RUN rustup default nightly
RUN apt-get update \
    && apt-get install -y ca-certificates tzdata libpq-dev sudo \
    && rm -rf /var/lib/apt/lists/*
RUN USER=root cargo new --bin api
COPY google-jwt-verify google-jwt-verify
WORKDIR ./api
ENV TZ=Etc/UTC \
    APP_USER=appuser
RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER
RUN echo "$APP_USER ALL=(ALL) ALL" > /etc/sudoers.d/$APP_USER && chmod 0440 /etc/sudoers.d/$APP_USER
RUN apt-get install sudo
RUN su
COPY api/Cargo.toml ./Cargo.toml
RUN cargo build --release
RUN rm src/*.rs
ADD api ./
RUN rm ./target/release/deps/api*
RUN cargo build --release
EXPOSE $PORT
RUN chown -R $APP_USER:$APP_USER .
RUN chmod u+x ./start_api.sh
USER $APP_USER
CMD ["sh", "./start_api.sh"]