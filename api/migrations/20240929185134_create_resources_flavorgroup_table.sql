CREATE TABLE `resources_flavorgroup` (
    -- TODO make this unsigned
    `id` bigint(20) NOT NULL AUTO_INCREMENT,
    `name` varchar(64) NOT NULL,
    `project_id` int(11) NOT NULL,
    PRIMARY KEY (`id`),
    UNIQUE KEY `name` (`name`),
    KEY `resources_flavorgroup_project_id_087d3ae0_fk_user_project_id` (`project_id`),
    CONSTRAINT `resources_flavorgroup_project_id_087d3ae0_fk_user_project_id` FOREIGN KEY (`project_id`) REFERENCES `user_project` (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=1 DEFAULT CHARSET=utf8
