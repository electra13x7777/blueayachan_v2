CREATE TABLE pictimeout
(
    id serial NOT NULL,
    user_id integer NOT NULL,
    last_pic TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    CONSTRAINT pictimeout_pkey PRIMARY KEY (id),
    FOREIGN KEY (user_id) REFERENCES blueayachanuser(id),
    UNIQUE(user_id)
);