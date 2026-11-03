import json
from pathlib import Path
import sqlite3
import tempfile
import unittest

import seed_log


class LogFixtureTests(unittest.TestCase):
    def test_seed_requires_owned_root_and_committed_sequence_and_does_not_modify_watermark(self):
        with tempfile.TemporaryDirectory(prefix="opencoders-live-") as tmp:
            root = Path(tmp).resolve()
            (root / "fixture-owner.json").write_text('{"pid": 123}')
            with sqlite3.connect(root / "server.sqlite") as db:
                db.executescript("CREATE TABLE event_sequence (aggregate_id TEXT PRIMARY KEY, seq INTEGER);"
                                 "CREATE TABLE event (id TEXT PRIMARY KEY,aggregate_id TEXT,seq INTEGER,created INTEGER,type TEXT,data TEXT);"
                                 "INSERT INTO event_sequence VALUES ('ses_test', 3);")
            event = {"type": "session.renamed", "id": "evt_test", "created": 1,
                     "durable": {"aggregateID": "ses_test", "seq": 2, "version": 1},
                     "data": {"sessionID": "ses_test", "title": "fixture"}}
            with self.assertRaisesRegex(ValueError, "owner"):
                seed_log.seed(root, 456, [event])
            bad = {**event, "durable": {**event["durable"], "seq": 4}}
            with self.assertRaisesRegex(ValueError, "not committed"):
                seed_log.seed(root, 123, [bad])
            seed_log.seed(root, 123, [event])
            with sqlite3.connect(root / "server.sqlite") as db:
                self.assertEqual(db.execute("SELECT seq FROM event_sequence").fetchone()[0], 3)
                row = db.execute("SELECT seq,type,data FROM event").fetchone()
                self.assertEqual(row[:2], (2, "session.renamed.1"))
                self.assertEqual(json.loads(row[2]), event["data"])

    def test_seed_rejects_database_symlink_outside_disposable_root(self):
        with tempfile.TemporaryDirectory(prefix="opencoders-live-") as tmp, tempfile.TemporaryDirectory() as other:
            root = Path(tmp).resolve()
            (root / "fixture-owner.json").write_text('{"pid": 123}')
            outside = Path(other) / "server.sqlite"
            outside.touch()
            (root / "server.sqlite").symlink_to(outside)
            with self.assertRaisesRegex(ValueError, "inside runner root"):
                seed_log.seed(root, 123, [])
