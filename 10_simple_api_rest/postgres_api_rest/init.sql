CREATE TABLE IF NOT EXISTS tasks (
    id SERIAL PRIMARY KEY,  -- SERIAL es un tipo de dato autoincremental en PostgreSQL
    title VARCHAR(255) NOT NULL,  -- Equivalente a String en Rust
    completed BOOLEAN NOT NULL DEFAULT FALSE,  -- Equivalente a bool en Rust
    creation_time TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,  -- Se rellena automáticamente al crear
    modification_time TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP  -- Se rellena automáticamente al crear
);

-- Crear un trigger para actualizar modification_time automáticamente al modificar la fila
CREATE OR REPLACE FUNCTION update_modification_time()
RETURNS TRIGGER AS $$
BEGIN
    NEW.modification_time = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER update_tasks_modification_time
BEFORE UPDATE ON tasks
FOR EACH ROW
EXECUTE FUNCTION update_modification_time();

-------------

INSERT INTO tasks (title, completed) VALUES ('Hacer la compra', FALSE);

SELECT * FROM tasks;

UPDATE tasks SET completed = TRUE WHERE id = 1;
SELECT * FROM tasks;