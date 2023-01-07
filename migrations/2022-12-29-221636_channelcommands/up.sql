CREATE TABLE channelcommands
(
    id serial NOT NULL,
    channel_bac_id integer NOT NULL,-- channel_twitch_id VARCHAR(255) NOT NULL, -- TWITCH ID STRING
    command_id integer NOT NULL,
    is_active boolean DEFAULT TRUE NOT NULL,
    is_broadcaster_only boolean DEFAULT FALSE NOT NULL,
    is_mod_only boolean DEFAULT FALSE NOT NULL,
    has_timeout boolean DEFAULT FALSE NOT NULL,
    timeout_dur integer DEFAULT 30 NOT NULL, -- COMMAND TIMEOUT IN SECONDS
    --num_used integer DEFAULT 0 NOT NULL, -- ammount used in channel
    last_updated TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    CONSTRAINT channelcommands_pkey PRIMARY KEY (channel_bac_id, command_id),
    FOREIGN KEY (channel_bac_id) REFERENCES blueayachanuser(id),--FOREIGN KEY (channel_twitch_id) REFERENCES blueayachanuser(twitch_id),
    FOREIGN KEY (command_id) REFERENCES blueayacommands(id),--CONSTRAINT channel_command
    UNIQUE (channel_bac_id, command_id)
);