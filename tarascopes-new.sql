-- Adminer 5.4.1 PostgreSQL 18.1 dump

DROP TABLE IF EXISTS "frames";
DROP SEQUENCE IF EXISTS frames_frameid_seq;
CREATE SEQUENCE frames_frameid_seq INCREMENT 1 MINVALUE 1 MAXVALUE 2147483647 START 2375 CACHE 1;

CREATE TABLE "public"."frames" (
    "kaleidoid" uuid NOT NULL,
    "frame_count" integer NOT NULL,
    "frameid" integer DEFAULT nextval('frames_frameid_seq') NOT NULL,
    "timestamp" timestamp DEFAULT CURRENT_TIMESTAMP
)
WITH (oids = false);

CREATE UNIQUE INDEX frames_unique ON public.frames USING btree (frameid);


DROP TABLE IF EXISTS "instagram_posts";
DROP SEQUENCE IF EXISTS instagram_posts_id_seq;
CREATE SEQUENCE instagram_posts_id_seq INCREMENT 1 MINVALUE 1 MAXVALUE 2147483647 CACHE 1;

CREATE TABLE "public"."instagram_posts" (
    "id" integer DEFAULT nextval('instagram_posts_id_seq') NOT NULL,
    "kaleido_id" uuid,
    "permalink" text NOT NULL,
    "catbox_link" text NOT NULL,
    CONSTRAINT "instagram_posts_pk" PRIMARY KEY ("id")
)
WITH (oids = false);

CREATE UNIQUE INDEX instagram_posts_unique ON public.instagram_posts USING btree (permalink);


DROP VIEW IF EXISTS "progress";
CREATE TABLE "progress" ("id" uuid, "count" bigint, "frame_count" json, "duration" interval);


DROP VIEW IF EXISTS "showcase";
CREATE TABLE "showcase" ("video" text, "gif" text, "thumbnail" text, "ts" timestamp, "parameters" json, "id" uuid);


DROP TABLE IF EXISTS "tarascope";
CREATE TABLE "public"."tarascope" (
    "id" uuid NOT NULL,
    "parameters" json NOT NULL,
    "timestamp" timestamp DEFAULT CURRENT_TIMESTAMP,
    "status" integer DEFAULT '0',
    CONSTRAINT "tarascope_pk" PRIMARY KEY ("id")
)
WITH (oids = false);


ALTER TABLE ONLY "public"."frames" ADD CONSTRAINT "frames_tarascope_fk" FOREIGN KEY (kaleidoid) REFERENCES tarascope(id) ON DELETE CASCADE NOT DEFERRABLE;

ALTER TABLE ONLY "public"."instagram_posts" ADD CONSTRAINT "instagram_posts_tarascope_fk" FOREIGN KEY (kaleido_id) REFERENCES tarascope(id) NOT DEFERRABLE;

DROP TABLE IF EXISTS "newview";
CREATE VIEW "newview" AS SELECT t.id,
    count(f.*) AS count,
    ((t.parameters -> 'frame'::text) -> '_frames_max'::text) AS "?column?"
   FROM (tarascope t
     JOIN frames f ON ((f.kaleidoid = t.id)))
  WHERE (t.status <> 3)
  GROUP BY t.id;

DROP TABLE IF EXISTS "progress";
CREATE VIEW "progress" AS SELECT t.id,
    count(f.*) AS count,
    ((t.parameters -> 'frames'::text) -> '_frames_max'::text) AS frame_count,
    (max(f."timestamp") - min(f."timestamp")) AS duration
   FROM (tarascope t
     JOIN frames f ON ((f.kaleidoid = t.id)))
  WHERE (t.status <> 3)
  GROUP BY t.id;

DROP TABLE IF EXISTS "showcase";
CREATE VIEW "showcase" AS SELECT concat(id, '/video.mp4') AS video,
    concat(id, '/video.gif') AS gif,
    concat(id, '/frame_00000.png') AS thumbnail,
    parameters,
    id,
    MAX(f.timestamp) - MIN(f.timestamp) AS duration,
    t.timestamp as ts
   FROM tarascope t
   JOIN frames f ON f.kaleidoid = t.id
  WHERE (status = 3)
  GROUP by t.id;

-- 2025-11-17 23:59:11 UTC
