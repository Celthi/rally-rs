DROP TABLE IF EXISTS rallytokens;

CREATE TABLE rallytokens
(
    user_name varchar(1024) PRIMARY KEY NOT NULL,
    token varchar(1024) NOT NULL
);
