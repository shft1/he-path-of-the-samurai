FROM debian:12-slim AS build

RUN apt-get update \
    && apt-get install -y --no-install-recommends fp-compiler ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /opt/legacy
COPY legacy.pas ./
RUN fpc -O2 -S2 legacy.pas

FROM alpine:3.20
RUN apk add --no-cache bash ca-certificates postgresql-client curl

ENV LEGACY_MODE=cron \
    LEGACY_CRON_SCHEDULE="*/5 * * * *"

COPY --from=build /opt/legacy/legacy /usr/local/bin/legacy
COPY run.sh /usr/local/bin/legacy-entrypoint.sh
RUN chmod +x /usr/local/bin/legacy-entrypoint.sh \
    && curl -fsSL https://github.com/aptible/supercronic/releases/download/v0.2.27/supercronic-linux-amd64 \
        -o /usr/local/bin/supercronic \
    && chmod +x /usr/local/bin/supercronic

ENTRYPOINT ["/usr/local/bin/legacy-entrypoint.sh"]
