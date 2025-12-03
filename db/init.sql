-- Basic schema

CREATE TABLE IF NOT EXISTS iss_fetch_log (
    id BIGSERIAL PRIMARY KEY,
    fetched_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    source_url TEXT NOT NULL,
    payload JSONB NOT NULL
);
CREATE INDEX IF NOT EXISTS idx_iss_fetch_log_fetched_at ON iss_fetch_log (fetched_at DESC);
CREATE INDEX IF NOT EXISTS idx_iss_fetch_log_source_time ON iss_fetch_log (source_url, fetched_at DESC);

CREATE TABLE IF NOT EXISTS telemetry_legacy (
    id BIGSERIAL PRIMARY KEY,
    recorded_at TIMESTAMPTZ NOT NULL,
    voltage NUMERIC(6,2) NOT NULL,
    temp NUMERIC(6,2) NOT NULL,
    source_file TEXT NOT NULL
);
CREATE INDEX IF NOT EXISTS idx_telemetry_legacy_recorded_at ON telemetry_legacy (recorded_at DESC);

CREATE TABLE IF NOT EXISTS cms_pages (
    id BIGSERIAL PRIMARY KEY,
    slug TEXT UNIQUE NOT NULL,
    title TEXT NOT NULL,
    body TEXT NOT NULL
);
CREATE INDEX IF NOT EXISTS idx_cms_pages_slug ON cms_pages (slug);

CREATE TABLE IF NOT EXISTS cms_blocks (
    id BIGSERIAL PRIMARY KEY,
    slug TEXT UNIQUE NOT NULL,
    content TEXT NOT NULL,
    is_active BOOLEAN NOT NULL DEFAULT TRUE
);
CREATE INDEX IF NOT EXISTS idx_cms_blocks_slug ON cms_blocks (slug);

CREATE MATERIALIZED VIEW IF NOT EXISTS telemetry_legacy_hourly AS
SELECT date_trunc('hour', recorded_at) AS bucket,
       avg(voltage) AS avg_voltage,
        avg(temp) AS avg_temp,
        count(*) AS sample_count
FROM telemetry_legacy
GROUP BY bucket;

-- Seed with deliberately unsafe content for XSS practice
INSERT INTO cms_pages(slug, title, body)
VALUES
('welcome', 'Добро пожаловать', '<h3>Демо контент</h3><p>Этот текст хранится в БД</p>'),
('unsafe', 'Небезопасный пример', '<script>console.log("XSS training")
</script><p>Если вы видите всплывашку значит защита не работает</p>')
ON CONFLICT DO NOTHING;

INSERT INTO cms_blocks(slug, content, is_active)
VALUES
('dashboard_experiment', '<div class="experiment">Dashboard Experiment Content</div>', TRUE)
ON CONFLICT DO NOTHING;
