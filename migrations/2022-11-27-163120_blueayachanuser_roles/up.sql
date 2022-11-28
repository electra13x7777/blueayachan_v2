CREATE TABLE blueayachanuser_roles
(
    id serial NOT NULL,
    user_id int,
    role_id int,
    created TIMESTAMP default CURRENT_TIMESTAMP,
    CONSTRAINT bacur_pkey PRIMARY KEY (id),
    FOREIGN KEY (user_id) REFERENCES blueayachanuser(id),
    FOREIGN KEY (role_id) REFERENCES roles(id),
    UNIQUE(user_id, role_id)
)