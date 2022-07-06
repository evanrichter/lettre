# Build Stage
FROM rustlang/rust:nightly as builder

## Add source code to the build stage.
ADD . /src
WORKDIR /src

RUN cd mayhem/client && \
    env RUSTFLAGS="-g -Cdebug-assertions=on" \
        cargo +nightly -Z sparse-registry build --release && \
    mv target/release/client /client

# Package Stage
FROM rustlang/rust:nightly

COPY --from=builder /client /
