CREATE TABLE blueayachanuser
(
    id serial NOT NULL,
    user_nick character varying(255) NOT NULL,
    num_commands integer NOT NULL,
    date_added TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT blueayachanuser_pkey PRIMARY KEY (id)
);