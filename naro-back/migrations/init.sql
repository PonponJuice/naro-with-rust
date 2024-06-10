
CREATE TABLE IF NOT EXISTS users 
(
    id VARCHAR(255) NOT NULL PRIMARY KEY,
    username VARCHAR(255), 
    display_id VARCHAR(255)
);

CREATE TABLE IF NOT EXISTS user_passwords
(
    display_id VARCHAR(255) PRIMARY KEY, 
    hashed_pass VARCHAR(255)
);