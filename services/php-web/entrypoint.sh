#!/bin/sh
set -e

# Wait for database to be ready
until php artisan db:monitor --databases=pgsql 2>/dev/null; do
  echo "Waiting for database..."
  sleep 2
done

# Run migrations
php artisan migrate --force

# Start PHP-FPM
exec php-fpm -F

