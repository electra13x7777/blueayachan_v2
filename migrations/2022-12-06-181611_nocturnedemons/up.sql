CREATE TABLE nocturnedemons
(
    id serial NOT NULL,
    demon_name character varying(255) NOT NULL,
    demon_img_link character varying(255) NOT NULL,
    CONSTRAINT nocturnedemons_pkey PRIMARY KEY (id)
);