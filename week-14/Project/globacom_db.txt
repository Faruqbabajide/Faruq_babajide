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
-- Name: dataplan; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.dataplan (
    data_id integer NOT NULL,
    data_size text NOT NULL,
    data_duration_day text NOT NULL,
    data_price_naira text NOT NULL
);


ALTER TABLE public.dataplan OWNER TO postgres;

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
-- Name: project; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.project (
    pno integer,
    pname text,
    pdura character(50),
    prmanagerid integer
);


ALTER TABLE public.project OWNER TO postgres;

--
-- Name: staff; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.staff (
    id integer NOT NULL,
    name text NOT NULL,
    no integer NOT NULL,
    sal real NOT NULL,
    age integer NOT NULL,
    phone integer
);


ALTER TABLE public.staff OWNER TO postgres;

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
-- Data for Name: dataplan; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.dataplan (data_id, data_size, data_duration_day, data_price_naira) FROM stdin;
1	350MB	2	200
2	1.8GB	14	500
3	3.9GB	30	1000
4	7.5GB	30	1500
5	9.2GB	30	2000
6	10.8GB	30	2500
6	14GB	30	30000
8	18GB	30	4000
9	24GB	30	5000
10	29.9GB	30	8000
11	50GB	30	10000
\.


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
-- Data for Name: project; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.project (pno, pname, pdura, prmanagerid) FROM stdin;
11	A	9 months                                          	102
22	C	14 months                                         	97
33	B	16 months                                         	120
44	D	25 months                                         	108
55	E	9 months                                          	107
\.


--
-- Data for Name: staff; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.staff (id, name, no, sal, age, phone) FROM stdin;
101	Alade joy	2	250000	33	802389832
100	Mustapha Ali	3	175000	32	806328531
100	Alokwe Martin	7	380000	32	806485831
97	Dankede Aminat	5	550000	48	804385831
97	Josiah Joshua	1	120000	40	804384531
102	Makinde Mary	2	450000	45	804384681
120	Adeleke Jane	4	200000	38	808984681
120	Osahon Mark	6	320000	44	802284681
120	Suleiman Ajayi	3	800000	50	802484681
122	Kuti Lawal	1	750000	35	802654681
\.


--
-- PostgreSQL database dump complete
--

