-- Create projects table
CREATE TABLE projects (
    project_id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    created_at TEXT NOT NULL
);

-- Create bugs table
CREATE TABLE bugs (
    bug_id TEXT PRIMARY KEY,
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    reported_by TEXT NOT NULL,
    severity TEXT NOT NULL,
    status TEXT NOT NULL,
    assigned_to TEXT,
    project_id TEXT,
    created_at TEXT NOT NULL,
    FOREIGN KEY (project_id) REFERENCES projects (project_id)
);