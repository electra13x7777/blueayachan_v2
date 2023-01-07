CREATE TABLE botchannels
(
    id serial NOT NULL,
    channel_name VARCHAR(255) NOT NULL,
    channel_twitch_id VARCHAR(255) NOT NULL,
    last_updated TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    CONSTRAINT botchannels_pkey PRIMARY KEY (id)
);