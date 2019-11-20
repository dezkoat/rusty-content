CREATE TABLE post (
  id CHAR(32) PRIMARY KEY,
  title VARCHAR NOT NULL,
  content TEXT NOT NULL,
  created_timestamp TIMESTAMPTZ default NOW() NOT NULL,
  updated_timestamp TIMESTAMPTZ default NOW() NOT NULL
);
