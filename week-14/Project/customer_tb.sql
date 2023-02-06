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
-- Name: customer; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.customer (
    c_id integer NOT NULL,
    c_name text NOT NULL,
    c_age integer NOT NULL,
    c_email text NOT NULL,
    c_mobile bigint NOT NULL,
    eid integer NOT NULL,
    data_id integer
);


ALTER TABLE public.customer OWNER TO postgres;

--
-- Data for Name: customer; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.customer (c_id, c_name, c_age, c_email, c_mobile, eid, data_id) FROM stdin;
110	Musta Karim 	35	m_karim@gmail.com	8055089112	102	5
111	Lilian  	45	l_jaiye@gmail.com	8055013422	100	3
112	Arthur Musa  	50	a_musa@gmail.com	8055864422	107	10
113	Philip Akonjo  	50	p_akonjo@gmail.com	9089864422	100	2
114	Marylene Mapa  	55	m_mapa@gmail.com	9089286722	120	5
115	Oghenero  	50	o_agor@gmail.com	9027985722	117	11
116	Adam Bree  	33	a_bree@gmail.com	9021379422	102	1
117	Okafor Mathias  	45	o_mathias@gmail.com	7021373792	120	10
118	Samson Adeleke  	65	s_adeleke@gmail.com	7021367592	170	11
119	Lawal Tamilore  	55	l_tamilore@gmail.com	7021333456	107	5
1201	James job 	44	j_job@gmail.com	7021235852	100	8
121	Matthew Jakade 	44	m_jakande@gmail.com	7021334872	120	2
122	Jimila Adegboye 	20	j_adegboye@gmail.com	7021334862	107	5
\.


--
-- PostgreSQL database dump complete
--

