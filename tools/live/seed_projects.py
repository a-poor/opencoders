"""Fixed project inventory fixture, restricted to the owned disposable smoke DB.

Seeds archive/order metadata only; it does not claim to exercise server archival.
"""
import json
import os
from pathlib import Path
import sqlite3
import sys
import tempfile
from urllib.parse import quote


def seed(root, pid, cleanup=False):
    root = Path(root).resolve(strict=True)
    if not root.name.startswith("opencoders-live-") or not root.is_relative_to(Path(tempfile.gettempdir()).resolve()):
        raise ValueError("fixture root must be a disposable runner directory")
    if json.loads((root / "fixture-owner.json").read_text()) != {"pid": pid}:
        raise ValueError("fixture owner does not match runner")
    database = (root / "server.sqlite").resolve(strict=True)
    if database.parent != root:
        raise ValueError("fixture database must stay inside runner root")
    with sqlite3.connect(f"file:{quote(str(database))}?mode=rw", uri=True) as db:
        for suffix, active, archived in [("a", 100, None), ("b", 100, None), ("c", 200, 300)]:
            name = "opencoders-setup-" + suffix
            directory = root / name
            if cleanup:
                db.execute("DELETE FROM project WHERE id=? AND worktree=?", (name, str(directory)))
            else:
                directory.mkdir(exist_ok=True)
                db.execute(
                    "INSERT INTO project (id,worktree,time_created,time_updated,time_active,time_archived,sandboxes) VALUES (?,?,?,?,?,?,?)",
                    (name, str(directory), 1, 1, active, archived, "[]"),
                )


if __name__ == "__main__":
    if sys.argv[1:] not in [[], ["--cleanup"]]:
        raise ValueError("unsupported fixture arguments")
    seed(os.environ["OPENCODERS_LIVE_ROOT"], int(os.environ["OPENCODERS_LIVE_PID"]), bool(sys.argv[1:]))
