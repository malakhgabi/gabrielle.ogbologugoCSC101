--
-- PostgreSQL database dump
--

\restrict 8oCdn0s9Uyye87fF6Qhqkkg0XQKjZFy0GRjSYboGQCibtpqNac8CRBvTWXP0u2V

-- Dumped from database version 18.6
-- Dumped by pg_dump version 18.6

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
-- Name: project; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.project (
    pno integer NOT NULL,
    pname character(50) NOT NULL,
    pduration character(50) NOT NULL,
    project_managerid integer NOT NULL
);


ALTER TABLE public.project OWNER TO postgres;

--
-- Data for Name: project; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.project (pno, pname, pduration, project_managerid) FROM stdin;
11	A                                                 	9 Months                                          	102
22	B                                                 	14 Months                                         	97
33	C                                                 	16 Months                                         	120
44	D                                                 	25 Months                                         	108
55	E                                                 	9 Months                                          	107
\.


--
-- Name: project project_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.project
    ADD CONSTRAINT project_pkey PRIMARY KEY (project_managerid);


--
-- PostgreSQL database dump complete
--

\unrestrict 8oCdn0s9Uyye87fF6Qhqkkg0XQKjZFy0GRjSYboGQCibtpqNac8CRBvTWXP0u2V

