CREATE TABLE data (
  id bigint PRIMARY KEY,
  data_json json NOT NULL,
  data_jsonb jsonb NOT NULL
);

INSERT INTO data (
  id,
  data_json,
  data_jsonb)
VALUES (
  1,
  '{"s": "abc","i": 123}',
  '{"s": "abc","i": 123}'),
(
  2,
  '{"s": "def","i": 456}',
  '{"s": "def","i": 456}');
