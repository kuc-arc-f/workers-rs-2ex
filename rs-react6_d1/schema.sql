CREATE TABLE todos (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  title TEXT NOT NULL,
  description TEXT,
  completed BOOLEAN DEFAULT 0,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL
);