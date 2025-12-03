from __future__ import annotations

import csv
import os
import random
import sys
import time
from dataclasses import dataclass
from datetime import datetime, timezone
from pathlib import Path

import psycopg


@dataclass
class TelemetryRow:
    recorded_at: datetime
    voltage: float
    temperature: float
    source_file: str

    def as_csv_row(self) -> list[str]:
        return [
            self.recorded_at.strftime("%Y-%m-%d %H:%M:%S"),
            f"{self.voltage:.2f}",
            f"{self.temperature:.2f}",
            self.source_file,
        ]


def generate_row(filename: str) -> TelemetryRow:
    return TelemetryRow(
        recorded_at=datetime.now(timezone.utc),
        voltage=random.uniform(3.2, 12.6),
        temperature=random.uniform(-50.0, 80.0),
        source_file=filename,
    )


def write_csv(row: TelemetryRow, directory: Path) -> Path:
    directory.mkdir(parents=True, exist_ok=True)
    path = directory / row.source_file
    with path.open("w", newline="") as fh:
        writer = csv.writer(fh)
        writer.writerow(["recorded_at", "voltage", "temp", "source_file"])
        writer.writerow(row.as_csv_row())
    return path


def copy_into_db(csv_path: Path) -> None:
    conn_str = (
        f"host={os.getenv('PGHOST', 'db')} "
        f"port={os.getenv('PGPORT', '5432')} "
        f"user={os.getenv('PGUSER', 'monouser')} "
        f"dbname={os.getenv('PGDATABASE', 'monolith')} "
        f"password={os.getenv('PGPASSWORD', 'monopass')}"
    )
    copy_sql = (
        "COPY telemetry_legacy(recorded_at, voltage, temp, source_file) "
        "FROM STDIN WITH (FORMAT csv, HEADER true)"
    )
    with psycopg.connect(conn_str) as conn, conn.cursor() as cur, csv_path.open("r") as fh:
        cur.copy(copy_sql, fh)
        conn.commit()


def run_cycle(csv_dir: Path) -> None:
    timestamp = datetime.now(timezone.utc).strftime("%Y%m%d_%H%M%S")
    filename = f"telemetry_{timestamp}.csv"
    row = generate_row(filename)
    csv_path = write_csv(row, csv_dir)
    copy_into_db(csv_path)
    print(f"[legacy-cli] wrote {csv_path}", flush=True)


def main() -> None:
    csv_dir = Path(os.getenv("CSV_OUT_DIR", "/data/csv"))
    run_once = os.getenv("LEGACY_RUN_ONCE", "true").lower() == "true"
    interval = int(os.getenv("GEN_PERIOD_SEC", "300"))

    while True:
        try:
            run_cycle(csv_dir)
        except Exception as exc:  # pylint: disable=broad-except
            print(f"[legacy-cli] error: {exc}", file=sys.stderr, flush=True)
        if run_once:
            break
        time.sleep(interval)


if __name__ == "__main__":
    main()

