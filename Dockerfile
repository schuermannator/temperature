FROM rust:1.46 as builder
WORKDIR build
COPY . .
RUN cargo build --release

FROM gcr.io/distroless/cc
COPY dist/ dist/
COPY index.html ./
COPY --from=builder /build/target/release/temperature ./
EXPOSE 8000
ENTRYPOINT ["./temperature"]
