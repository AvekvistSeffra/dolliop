-- Your SQL goes here
CREATE TABLE users_to_lists(
  id int AUTO_INCREMENT NOT NULL,
  list int NOT NULL,
  user int NOT NULL,
  CONSTRAINT users_to_lists_pk PRIMARY KEY (id)
);