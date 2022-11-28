CREATE TABLE dreamboumtweets
(
    id serial NOT NULL,
    tweet character varying(1024) NOT NULL,
    tweet_date character varying(255) NOT NULL,
    CONSTRAINT dreamboumtweets_pkey PRIMARY KEY (id)
);