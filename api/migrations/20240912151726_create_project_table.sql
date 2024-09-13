CREATE TABLE project (
    id INT UNSIGNED NOT NULL AUTO_INCREMENT PRIMARY KEY,
    name VARCHAR(255) UNIQUE NOT NULL,
    -- TODO shorten to 36 chars or maybe use uuid type
    openstack_id VARCHAR(255) UNIQUE NOT NULL,
    -- TODO replace by enum
    user_class SMALLINT UNSIGNED NOT NULL
);
