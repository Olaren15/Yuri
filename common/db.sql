DROP DATABASE IF EXISTS yuri;
CREATE DATABASE IF NOT EXISTS yuri DEFAULT CHARACTER SET utf8 COLLATE utf8_unicode_ci;
USE yuri;

CREATE TABLE settings (
	id             int          NOT NULL AUTO_INCREMENT,
	weight         int          NOT NULL,
	debug_token    varchar(255) NOT NULL,
	release_token  varchar(255) NOT NULL,
	command_prefix varchar(1)   NOT NULL,
	PRIMARY KEY (id)
);

CREATE TABLE commands (
	id              int NOT NULL AUTO_INCREMENT,
	name            varchar(255) DEFAULT NULL,
	description     varchar(255) DEFAULT NULL,
	everyone_text   varchar(255) DEFAULT NULL,
	nobody_text     varchar(255) DEFAULT NULL,
	one_person_text varchar(255) DEFAULT NULL,
	is_nsfw         bool         DEFAULT FALSE,
	PRIMARY KEY (id)
);

CREATE TABLE images (
	id         int          NOT NULL AUTO_INCREMENT,
	command_id int          NOT NULL,
	url        varchar(500) NOT NULL,
	PRIMARY KEY (id),
	FOREIGN KEY (command_id) REFERENCES commands (id)
);
