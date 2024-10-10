CREATE TABLE `quota_flavorquota` (
    -- TODO: make this unsigned
    `quota_ptr_id` bigint(20) NOT NULL,
    -- TODO: make this unsigned
    `flavor_group_id` bigint(20) NOT NULL,
    PRIMARY KEY (`quota_ptr_id`),
    KEY `quota_flavorquota_flavor_group_id_1296e59d_fk_resources` (`flavor_group_id`),
    CONSTRAINT `quota_flavorquota_flavor_group_id_1296e59d_fk_resources` FOREIGN KEY (`flavor_group_id`) REFERENCES `resources_flavorgroup` (`id`),
    CONSTRAINT `quota_flavorquota_quota_ptr_id_466c4233_fk_quota_quota_id` FOREIGN KEY (`quota_ptr_id`) REFERENCES `quota_quota` (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8
