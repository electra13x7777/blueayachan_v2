CREATE TABLE BAC_user_demons
(
    id serial NOT NULL,
    user_id integer NOT NULL,
    saved_demon_id integer NOT NULL DEFAULT 62, -- DEFAULT SLIME
    saved_demon_rarity integer NOT NULL DEFAULT 1, -- DEFAULT 1 STAR
    last_demon_id integer NOT NULL DEFAULT 62, -- DEFAULT SLIME
    last_demon_rarity integer NOT NULL DEFAULT 1, -- DEFAULT 1 STAR

    CONSTRAINT bacud_pkey PRIMARY KEY (id),
    FOREIGN KEY (user_id) REFERENCES blueayachanuser(id),
    UNIQUE(user_id)
);