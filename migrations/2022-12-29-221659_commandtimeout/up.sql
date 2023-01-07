CREATE TABLE commandtimeout
(
    id serial NOT NULL,
    --channel_twitch_id VARCHAR(255) NOT NULL, -- TWITCH ID STRING
    --user_twitch_id VARCHAR(255) NOT NULL, -- EITHER BOT ID OR TWITCH ID STRING
    channel_bac_id integer NOT NULL,
    user_bac_id integer NOT NULL,
    command_id integer NOT NULL,
    last_command TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    CONSTRAINT commandtimeout_pkey PRIMARY KEY (channel_bac_id, user_bac_id, command_id),--(channel_twitch_id, user_twitch_id, command_id),
    --FOREIGN KEY (channel_twitch_id) REFERENCES blueayachanuser(twitch_id),
    --FOREIGN KEY (user_twitch_id) REFERENCES blueayachanuser(twitch_id),
    FOREIGN KEY (channel_bac_id) REFERENCES blueayachanuser(id),
    FOREIGN KEY (user_bac_id) REFERENCES blueayachanuser(id),
    --CONSTRAINT channel_chatter UNIQUE (channel_twitch_id, user_twitch_id),
    FOREIGN KEY (command_id) REFERENCES blueayacommands(id),
    --UNIQUE (channel_twitch_id, user_twitch_id, command_id)
    UNIQUE (channel_bac_id, user_bac_id, command_id)
);