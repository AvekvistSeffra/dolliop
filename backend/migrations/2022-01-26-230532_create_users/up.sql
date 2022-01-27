-- Your SQL goes here
CREATE TABLE users(
  id int AUTO_INCREMENT NOT NULL,
  username VARCHAR(255) NOT NULL,
  password VARCHAR(255) NOT NULL,
  CONSTRAINT users_pk PRIMARY KEY (id)
);