FROM alpine:latest

ARG VERSION
LABEL org.opencontainers.image.version=$VERSION
LABEL org.opencontainers.image.source="https://github.com/$GITHUB_REPOSITORY"

# Copy the appropriate binary based on the target platform
COPY artifacts/duoload-linux-* /usr/local/bin/duoload
RUN chmod +x /usr/local/bin/duoload

ENTRYPOINT ["/usr/local/bin/duoload"]
