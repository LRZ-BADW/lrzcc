CREATE TABLE `pricing_flavorprice` (
    -- TODO: make this unsigned
    `id` int(11) NOT NULL AUTO_INCREMENT,
    `user_class` smallint(5) unsigned NOT NULL,
    `unit_price` double NOT NULL,
    `start_time` datetime(6) NOT NULL,
    -- TODO: make this unsigned
    `flavor_id` bigint(20) NOT NULL,
    PRIMARY KEY (`id`),
    UNIQUE KEY `pricing_flavorprice_user_class_start_time_fl_90aa18fe_uniq` (`user_class`,`start_time`,`flavor_id`),
    KEY `pricing_flavorprice_flavor_id_5d67d3e4_fk_resources_flavor_id` (`flavor_id`),
    CONSTRAINT `pricing_flavorprice_flavor_id_5d67d3e4_fk_resources_flavor_id` FOREIGN KEY (`flavor_id`) REFERENCES `resources_flavor` (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=1 DEFAULT CHARSET=utf8
