FROM debian:stable-slim

ARG VERSION
ARG TARGETARCH
LABEL org.opencontainers.image.version=$VERSION
LABEL org.opencontainers.image.source="https://github.com/$GITHUB_REPOSITORY"

RUN apt-get update \
    && apt install libssl3 \
    && rm -rf /var/lib/cache/apt/* /var/lib/cache/dpkg/*

# Copy the appropriate binary based on the target platform
COPY artifacts/duoload-${TARGETARCH}-unknown-linux-gnu-v${VERSION}/duoload /usr/local/bin/duoload
RUN chmod +x /usr/local/bin/duoload

ENTRYPOINT ["/usr/local/bin/duoload"]
