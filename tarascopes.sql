--
-- PostgreSQL database dump
--

-- \restrict psRxaer1jsSkbnqeYHvVjOZ9U9sLRun1rW7aZa1x3VJnDxNPnGMt11corxZxjXF

-- Dumped from database version 18.0 (Debian 18.0-1.pgdg13+3)
-- Dumped by pg_dump version 18.0 (Debian 18.0-1.pgdg13+3)

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET transaction_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: frames; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.frames (
    kaleidoid uuid NOT NULL,
    frame_count integer NOT NULL,
    frameid integer NOT NULL,
    "timestamp" timestamp without time zone DEFAULT CURRENT_TIMESTAMP
);


ALTER TABLE public.frames OWNER TO postgres;

--
-- Name: frames_frameid_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.frames_frameid_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.frames_frameid_seq OWNER TO postgres;

--
-- Name: frames_frameid_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.frames_frameid_seq OWNED BY public.frames.frameid;


--
-- Name: tarascope; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.tarascope (
    id uuid NOT NULL,
    parameters json NOT NULL,
    "timestamp" timestamp without time zone DEFAULT CURRENT_TIMESTAMP,
    status integer DEFAULT 0
);


ALTER TABLE public.tarascope OWNER TO postgres;

--
-- Name: showcase; Type: VIEW; Schema: public; Owner: postgres
--

CREATE VIEW public.showcase AS
 SELECT concat(id, '/video.mp4') AS video,
    concat(id, '/video.gif') AS gif,
    concat(id, '/frame_00000.png') AS thumbnail,
    "timestamp" AS ts,
    parameters,
    id
   FROM public.tarascope
  WHERE (status = 3);


ALTER VIEW public.showcase OWNER TO postgres;

--
-- Name: frames frameid; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.frames ALTER COLUMN frameid SET DEFAULT nextval('public.frames_frameid_seq'::regclass);


--
-- Name: frames frames_unique; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.frames
    ADD CONSTRAINT frames_unique UNIQUE (frameid);


--
-- Name: tarascope tarascope_pk; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.tarascope
    ADD CONSTRAINT tarascope_pk PRIMARY KEY (id);


--
-- Name: frames frames_tarascope_fk; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.frames
    ADD CONSTRAINT frames_tarascope_fk FOREIGN KEY (kaleidoid) REFERENCES public.tarascope(id) ON DELETE CASCADE;


--
-- PostgreSQL database dump complete
--

--\unrestrict psRxaer1jsSkbnqeYHvVjOZ9U9sLRun1rW7aZa1x3VJnDxNPnGMt11corxZxjXF

