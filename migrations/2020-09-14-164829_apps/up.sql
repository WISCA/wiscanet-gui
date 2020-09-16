CREATE TABLE applications (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name VARCHAR NOT NULL,
    op_mode VARCHAR NOT NULL,
    mac_mode VARCHAR NOT NULL,
    lang VARCHAR NOT NULL,
    matlab_dir TEXT NOT NULL,
    matlab_func TEXT NOT NULL,
    matlab_log TEXT NOT NULL,
    num_samples INTEGER NOT NULL,
    sample_rate REAL NOT NULL,
    freq REAL NOT NULL,
    bw REAL NOT NULL
)
