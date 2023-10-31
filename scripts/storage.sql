CREATE TABLE IF NOT EXISTS bckt_links (
  id SERIAL PRIMARY KEY,
  owner_email VARCHAR(64) NOT NULL,
  link_hash VARCHAR(8) NOT NULL,
  link_long TEXT NOT NULL
);