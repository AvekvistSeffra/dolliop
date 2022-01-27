-- Your SQL goes here
CREATE TABLE entries(
  id int AUTO_INCREMENT NOT NULL,
  item int NOT NULL,
  list int NOT NULL,
  amount int NOT NULL DEFAULT 0,
  CONSTRAINT entries_pk PRIMARY KEY (id)
);