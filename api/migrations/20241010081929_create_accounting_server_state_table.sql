CREATE TABLE `accounting_serverstate` (
    -- TODO: make this unsigned
    `state_ptr_id` int(11) NOT NULL,
    `instance_id` varchar(36) NOT NULL,
    `instance_name` varchar(255) NOT NULL,
    `status` varchar(18) NOT NULL,
    -- TODO: make this unsigned
    `flavor_id` bigint(20) NOT NULL,
    -- TODO: make this unsigned
    `user_id` int(11) NOT NULL,
    PRIMARY KEY (`state_ptr_id`),
    KEY `accounting_serverstate_flavor_id_77784f3e_fk_resources_flavor_id` (`flavor_id`),
    KEY `accounting_serverstate_user_id_9b6604e8_fk_user_user_id` (`user_id`),
    CONSTRAINT `accounting_serversta_state_ptr_id_b5dee040_fk_accountin` FOREIGN KEY (`state_ptr_id`) REFERENCES `accounting_state` (`id`),
    CONSTRAINT `accounting_serverstate_flavor_id_77784f3e_fk_resources_flavor_id` FOREIGN KEY (`flavor_id`) REFERENCES `resources_flavor` (`id`),
    CONSTRAINT `accounting_serverstate_user_id_9b6604e8_fk_user_user_id` FOREIGN KEY (`user_id`) REFERENCES `user_user` (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8
