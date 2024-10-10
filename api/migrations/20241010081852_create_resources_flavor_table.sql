CREATE TABLE `resources_flavor` (
    -- TODO: make this unsigned
    `id` bigint(20) NOT NULL AUTO_INCREMENT,
    `name` varchar(64) NOT NULL,
    `openstack_id` varchar(255) NOT NULL,
    `weight` smallint(5) unsigned NOT NULL,
    -- TODO: make this unsigned
    `group_id` bigint(20) DEFAULT NULL,
    PRIMARY KEY (`id`),
    UNIQUE KEY `name` (`name`),
    UNIQUE KEY `openstack_id` (`openstack_id`),
    KEY `resources_flavor_group_id_e01c5178_fk_resources_flavorgroup_id` (`group_id`),
    CONSTRAINT `resources_flavor_group_id_e01c5178_fk_resources_flavorgroup_id` FOREIGN KEY (`group_id`) REFERENCES `resources_flavorgroup` (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=1 DEFAULT CHARSET=utf8
