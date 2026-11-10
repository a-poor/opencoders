import json
from pathlib import Path
import sqlite3
import tempfile
import unittest
import seed_projects


class ProjectFixtureTests(unittest.TestCase):
    def test_requires_owner_and_preserves_unrelated_rows_on_cleanup(self):
        with tempfile.TemporaryDirectory(prefix="opencoders-live-") as tmp:
            root = Path(tmp).resolve()
            (root / "fixture-owner.json").write_text(json.dumps({"pid": 123}))
            with sqlite3.connect(root / "server.sqlite") as db:
                db.execute("CREATE TABLE project(id TEXT PRIMARY KEY,worktree TEXT,time_created INTEGER,time_updated INTEGER,time_active INTEGER,time_archived INTEGER,sandboxes TEXT)")
                db.execute("INSERT INTO project(id,worktree) VALUES ('other','/other')")
            with self.assertRaisesRegex(ValueError, "owner"):
                seed_projects.seed(root, 456)
            seed_projects.seed(root, 123)
            with sqlite3.connect(root / "server.sqlite") as db:
                self.assertEqual(db.execute("SELECT count(*) FROM project WHERE time_archived IS NOT NULL").fetchone()[0], 1)
            seed_projects.seed(root, 123, cleanup=True)
            with sqlite3.connect(root / "server.sqlite") as db:
                self.assertEqual(db.execute("SELECT id FROM project").fetchall(), [("other",)])

    def test_rejects_database_outside_owned_root(self):
        with tempfile.TemporaryDirectory(prefix="opencoders-live-") as tmp, tempfile.TemporaryDirectory() as other:
            root = Path(tmp).resolve()
            (root / "fixture-owner.json").write_text('{"pid":123}')
            outside = Path(other) / "server.sqlite"
            outside.touch()
            (root / "server.sqlite").symlink_to(outside)
            with self.assertRaisesRegex(ValueError, "inside runner root"):
                seed_projects.seed(root, 123)
