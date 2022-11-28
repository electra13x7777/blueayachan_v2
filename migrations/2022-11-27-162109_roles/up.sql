CREATE TABLE roles
(
    id serial NOT NULL,
    role_name character varying(30),
    date_added TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT roles_pkey PRIMARY KEY (id)
);