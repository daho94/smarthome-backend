CREATE TABLE dashboards (
  id SERIAL PRIMARY KEY,
  user_id SERIAL references users(id) NOT NULL,
  settings JSONB
)