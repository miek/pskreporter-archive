CREATE TABLE reports (
  id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
  receiver_callsign VARCHAR NOT NULL,
  receiver_locator VARCHAR NOT NULL,
  sender_callsign VARCHAR NOT NULL,
  sender_locator VARCHAR NOT NULL,
  frequency BIGINT NOT NULL,
  timestamp DATETIME NOT NULL,
  mode VARCHAR NOT NULL,
  is_sender INTEGER NOT NULL,
  is_receiver INTEGER NOT NULL,
  sender_region VARCHAR NOT NULL,
  sender_dxcc VARCHAR NOT NULL,
  sender_dxcc_code VARCHAR NOT NULL,
  sender_dxcc_locator VARCHAR NOT NULL,
  snr INTEGER NOT NULL
);
