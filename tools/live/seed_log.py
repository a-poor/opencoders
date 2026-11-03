"""Seed captured rename events into the owned disposable DB for log replay tests.

v2.2.5's CLI does not enable payload retention. This fixture adapter does not
change projections or sequence counters, and does not claim normal retention.
"""
import json
import os
from pathlib import Path
import sqlite3
import sys
import tempfile
from urllib.parse import quote


def seed(root, pid, events):
    root = Path(root).resolve(strict=True)
    if not root.name.startswith("opencoders-live-") or not root.is_relative_to(Path(tempfile.gettempdir()).resolve()):
        raise ValueError("fixture root must be a disposable runner directory")
    owner = json.loads((root / "fixture-owner.json").read_text())
    if owner != {"pid": pid}:
        raise ValueError("fixture owner does not match runner")
    database = (root / "server.sqlite").resolve(strict=True)
    if database.parent != root:
        raise ValueError("fixture database must stay inside runner root")
    # rw prevents accidentally creating a new database if the pinned layout changes.
    with sqlite3.connect(f"file:{quote(str(database))}?mode=rw", uri=True) as db:
        db.execute("PRAGMA foreign_keys = ON")
        for event in events:
            durable = event["durable"]
            session_id = durable["aggregateID"]
            seq = durable["seq"]
            if (event["type"] != "session.renamed" or durable["version"] != 1
                    or event["data"]["sessionID"] != session_id
                    or not session_id.startswith("ses") or not event["id"].startswith("evt_")
                    or type(seq) is not int or not 0 <= seq <= 9_007_199_254_740_991):
                raise ValueError("fixture must be a captured durable rename")
            row = db.execute("SELECT seq FROM event_sequence WHERE aggregate_id = ?", (session_id,)).fetchone()
            if row is None or seq > row[0]:
                raise ValueError("fixture sequence was not committed by this server")
            db.execute(
                "INSERT INTO event (id,aggregate_id,seq,created,type,data) VALUES (?,?,?,?,?,?)",
                (event["id"], session_id, seq, event["created"], "session.renamed.1", json.dumps(event["data"])),
            )


if __name__ == "__main__":
    seed(os.environ["OPENCODERS_LIVE_ROOT"], int(os.environ["OPENCODERS_LIVE_PID"]), json.load(sys.stdin))
