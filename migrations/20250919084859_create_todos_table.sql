-- migrations/{timestamp}_create_todos_table.sql
CREATE TABLE IF NOT EXISTS todos (
    id INTEGER PRIMARY KEY NOT NULL,
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    completed BOOLEAN DEFAULT FALSE NOT NULL
);