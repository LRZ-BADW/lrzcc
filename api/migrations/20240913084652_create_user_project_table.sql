CREATE TABLE `user_project` (
    -- TODO: make this unsigned
    `id` int(11) NOT NULL AUTO_INCREMENT,
    `name` varchar(255) NOT NULL,
    -- TODO: shorten to 36 chars or maybe use uuid type
    `openstack_id` varchar(255) NOT NULL,
    -- TODO: replace by enum
    `user_class` smallint(5) unsigned NOT NULL,
    PRIMARY KEY (`id`),
    UNIQUE KEY `name` (`name`),
    UNIQUE KEY `openstack_id` (`openstack_id`)
) ENGINE=InnoDB AUTO_INCREMENT=1 DEFAULT CHARSET=utf8
