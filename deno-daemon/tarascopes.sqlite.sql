-- SQLite port of the PostgreSQL schema in tarascopes.sql.
-- UUIDs are stored as TEXT; JSON values are stored as validated JSON TEXT.

PRAGMA foreign_keys = ON;

CREATE TABLE IF NOT EXISTS tarascope (
    id TEXT NOT NULL PRIMARY KEY,
    parameters TEXT NOT NULL CHECK (json_valid(parameters)),
    "timestamp" TEXT DEFAULT CURRENT_TIMESTAMP,
    status INTEGER DEFAULT 0
);

CREATE TABLE IF NOT EXISTS frames (
    kaleidoid TEXT NOT NULL,
    frame_count INTEGER NOT NULL,
    -- PostgreSQL's owned sequence is represented by SQLite rowid autoincrement.
    frameid INTEGER PRIMARY KEY AUTOINCREMENT,
    "timestamp" TEXT DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT frames_unique UNIQUE (frameid),
    CONSTRAINT frames_tarascope_fk FOREIGN KEY (kaleidoid)
        REFERENCES tarascope(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS "hooks" (
	"id"	TEXT NOT NULL,
	"hook_url"	TEXT NOT NULL,
	PRIMARY KEY("id"),
	FOREIGN KEY("id") REFERENCES "tarascope"("id")
);

CREATE VIEW IF NOT EXISTS showcase AS
SELECT
    id || '/video.mp4' AS video,
    id || '/video.gif' AS gif,
    id || '/frame_00000.png' AS thumbnail,
    "timestamp" AS ts,
    parameters,
    id
FROM tarascope
WHERE status = 3;

-- The original dump first created null-only placeholder versions of these
-- views, then replaced them with these definitions. Only the final versions
-- are included here.
CREATE VIEW IF NOT EXISTS newview AS
SELECT
    t.id,
    count(*) AS count,
    json_extract(t.parameters, '$.frame._frames_max') AS "?column?"
FROM tarascope AS t
JOIN frames AS f ON f.kaleidoid = t.id
WHERE t.status <> 3
GROUP BY t.id;

CREATE VIEW IF NOT EXISTS progress AS
SELECT
    t.id,
    count(*) AS count,
    json_extract(t.parameters, '$.frames._frames_max') AS frame_count
FROM tarascope AS t
JOIN frames AS f ON f.kaleidoid = t.id
WHERE t.status <> 3
GROUP BY t.id;
