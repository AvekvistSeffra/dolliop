-- Your SQL goes here
CREATE TABLE items(
  id int AUTO_INCREMENT NOT NULL,
  name VARCHAR(255) NOT NULL,
  price FLOAT NOT NULL,
  CONSTRAINT items_pk PRIMARY KEY (id)
);