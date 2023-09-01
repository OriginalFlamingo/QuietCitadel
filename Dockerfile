FROM debian:12
COPY target/release/stage1 secret_url.txt /app/
COPY static /app/static/
ENV SECRET_PASSWORD=
WORKDIR /app
CMD ["/app/stage1"]
