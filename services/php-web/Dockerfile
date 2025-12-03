# syntax=docker/dockerfile:1.6

ARG LARAVEL_VERSION=11.*

FROM composer:2 AS app
ARG LARAVEL_VERSION
ENV COMPOSER_ALLOW_SUPERUSER=1
WORKDIR /var/www/html
RUN if command -v apk >/dev/null 2>&1; then \
      apk add --no-cache rsync git unzip; \
    else \
      apt-get update && apt-get install -y --no-install-recommends rsync git unzip && rm -rf /var/lib/apt/lists/*; \
    fi
RUN --mount=type=cache,target=/root/.composer \
    composer create-project --no-progress --prefer-dist laravel/laravel:${LARAVEL_VERSION} .
# Remove APP_KEY from .env so Laravel reads it from environment variables
RUN sed -i '/^APP_KEY=/d' .env
COPY laravel-patches/ /tmp/patches
RUN rsync -a /tmp/patches/ ./ && rm -rf /tmp/patches
RUN --mount=type=cache,target=/root/.composer \
    composer install --no-dev --optimize-autoloader
RUN find storage bootstrap/cache -type d -exec chmod 775 {} + \
    && chown -R www-data:www-data storage bootstrap/cache

FROM php:8.4-fpm-alpine AS php_runtime
ENV APP_ENV=production
RUN --mount=type=cache,target=/var/cache/apk \
    apk add --no-cache bash icu-data-full icu-libs postgresql-libs git \
    && apk add --no-cache --virtual .build-deps icu-dev libzip-dev oniguruma-dev postgresql-dev \
    && docker-php-ext-configure intl \
    && docker-php-ext-install -j$(nproc) pdo pdo_pgsql opcache intl \
    && apk del .build-deps
WORKDIR /var/www/html
COPY --from=app /var/www/html /var/www/html
COPY entrypoint.sh /usr/local/bin/entrypoint.sh
RUN chmod +x /usr/local/bin/entrypoint.sh \
    && chown -R www-data:www-data /var/www/html \
    && sed -i 's|pm.max_children =.*|pm.max_children = 8|' /usr/local/etc/php-fpm.d/zz-docker.conf \
    && sed -i 's|pm.start_servers =.*|pm.start_servers = 2|' /usr/local/etc/php-fpm.d/zz-docker.conf \
    && sed -i 's|pm.min_spare_servers =.*|pm.min_spare_servers = 2|' /usr/local/etc/php-fpm.d/zz-docker.conf \
    && sed -i 's|pm.max_spare_servers =.*|pm.max_spare_servers = 4|' /usr/local/etc/php-fpm.d/zz-docker.conf
EXPOSE 9000
CMD ["/usr/local/bin/entrypoint.sh"]

FROM nginx:1.27-alpine AS nginx_runtime
WORKDIR /var/www/html
COPY --from=app /var/www/html/public /var/www/html
COPY nginx.conf /etc/nginx/conf.d/default.conf
