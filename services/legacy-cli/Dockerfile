FROM python:3.12-alpine

ENV PYTHONUNBUFFERED=1

WORKDIR /app

RUN apk add --no-cache curl ca-certificates \
    && curl -fsSL https://github.com/aptible/supercronic/releases/download/v0.2.27/supercronic-linux-amd64 \
        -o /usr/local/bin/supercronic \
    && chmod +x /usr/local/bin/supercronic \
    && apk del curl

COPY requirements.txt .
RUN pip install --no-cache-dir -r requirements.txt

COPY app ./app
COPY entrypoint.sh /usr/local/bin/entrypoint.sh
RUN chmod +x /usr/local/bin/entrypoint.sh

ENTRYPOINT ["/usr/local/bin/entrypoint.sh"]

