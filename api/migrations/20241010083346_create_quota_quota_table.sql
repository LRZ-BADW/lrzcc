CREATE TABLE `quota_quota` (
    -- TODO: make this unsigned
    `id` bigint(20) NOT NULL AUTO_INCREMENT,
    `quota` int(11) NOT NULL,
    `user_id` int(11) NOT NULL,
    PRIMARY KEY (`id`),
    KEY `quota_quota_user_id_447e848b_fk_user_user_id` (`user_id`),
    CONSTRAINT `quota_quota_user_id_447e848b_fk_user_user_id` FOREIGN KEY (`user_id`) REFERENCES `user_user` (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=1 DEFAULT CHARSET=utf8
