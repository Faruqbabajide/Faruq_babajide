--
-- PostgreSQL database dump
--

-- Dumped from database version 15.1
-- Dumped by pg_dump version 15.1

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
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
-- Name: department; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.department (
    id integer,
    dno integer,
    name text,
    location character(50),
    pno integer
);


ALTER TABLE public.department OWNER TO postgres;

--
-- Data for Name: department; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.department (id, dno, name, location, pno) FROM stdin;
108	1	Administration	Ikeja                                             	44
101	2	Account	Egbeda                                            	11
120	4	Research	V.I                                               	33
100	3	Packaging	Ajah                                              	44
97	5	Account	Magodo                                            	22
122	6	Operations	Mile 2                                            	44
107	7	Packaging	Ketu                                              	55
\.


--
-- PostgreSQL database dump complete
--

