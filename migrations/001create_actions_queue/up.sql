CREATE TABLE IF NOT EXISTS actions_queue (
  id VARCHAR PRIMARY KEY NOT NULL,
  action_type VARCHAR NOT NULL,
  data VARCHAR NOT NULL,
  created_at timestamp with TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);