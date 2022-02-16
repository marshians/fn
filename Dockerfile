# syntax=docker/dockerfile:1.3
FROM node AS ui

COPY ui /ui

WORKDIR /ui

RUN --mount=type=cache,target=/.npm npm set cache /.npm && \
	export NODE_OPTIONS=--openssl-legacy-provider && \
	yarn install && \
	yarn build

FROM messense/rust-musl-cross:x86_64-musl AS rust

COPY . /src

WORKDIR /src

RUN rustc --version
RUN --mount=type=cache,target=/src/target cargo build --release \
	--target x86_64-unknown-linux-musl --bin main && \
    cp target/x86_64-unknown-linux-musl/release/main /main

FROM scratch

COPY --from=ui /ui/build /ui
COPY --from=rust /main /main
COPY words.txt /

EXPOSE 8080

ENV ROCKET_ADDRESS "0.0.0.0"
ENV ROCKET_PORT "8080"

ENTRYPOINT ["/main"]
