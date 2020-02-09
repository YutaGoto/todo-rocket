CREATE TABLE todos (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    description VARCHAR NOT NULL,
    completed BOOLEAN NOT NULL DEFAULT 0
);

INSERT INTO todos (description) VALUES ("demo task");
INSERT INTO todos (description) VALUES ("demo task2");
