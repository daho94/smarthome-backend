CREATE TABLE dashboards (
  id SERIAL PRIMARY KEY,
  user_id SERIAL references users(id) NOT NULL,
  name VARCHAR NOT NULL,
  default_dashboard Boolean NOT NULL,
  settings JSONB NOT NULL
)