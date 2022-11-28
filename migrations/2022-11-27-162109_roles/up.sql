CREATE TABLE roles
(
    id serial NOT NULL,
    role_name character varying(30) NOT NULL,
    date_added TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    CONSTRAINT roles_pkey PRIMARY KEY (id)
);