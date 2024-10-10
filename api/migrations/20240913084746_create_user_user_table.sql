CREATE TABLE `user_user` (
    -- TODO: make this unsigned
    `id` int(11) NOT NULL AUTO_INCREMENT,
    `password` varchar(128) NOT NULL,
    `last_login` datetime(6) DEFAULT NULL,
    `name` varchar(255) NOT NULL,
    -- TODO: shorten to 36 chars or maybe use uuid type
    `openstack_id` varchar(255) NOT NULL,
    -- TODO: replace by enum
    `role` smallint(5) unsigned NOT NULL,
    `is_staff` tinyint(1) NOT NULL,
    `is_active` tinyint(1) NOT NULL,
    -- TODO: make this unsigned
    `project_id` int(11) DEFAULT NULL,
    PRIMARY KEY (`id`),
    UNIQUE KEY `name` (`name`),
    UNIQUE KEY `openstack_id` (`openstack_id`),
    KEY `user_user_project_id_4053d423_fk_user_project_id` (`project_id`),
    CONSTRAINT `user_user_project_id_4053d423_fk_user_project_id` FOREIGN KEY (`project_id`) REFERENCES `user_project` (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=1 DEFAULT CHARSET=utf8
