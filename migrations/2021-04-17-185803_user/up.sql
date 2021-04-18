-- Your SQL goes here
CREATE TABLE users (
  email VARCHAR PRIMARY KEY,  
  hash VARCHAR NOT NULL,
  firstname VARCHAR NOT NULL,
  lastname VARCHAR NOT NULL,
  groups VARCHAR NOT NULL,
  pfp_link VARCHAR NOT NULL
)
