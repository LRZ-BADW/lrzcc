CREATE TABLE `budgeting_userbudget` (
    -- TODO: make this unsigned
    `id` int(11) NOT NULL AUTO_INCREMENT,
    `year` smallint(5) unsigned NOT NULL,
    `amount` int(10) unsigned NOT NULL,
    -- TODO: make this unsigned
    `user_id` int(11) NOT NULL,
    PRIMARY KEY (`id`),
    UNIQUE KEY `budgeting_userbudget_year_user_id_1a9a0366_uniq` (`year`,`user_id`),
    KEY `budgeting_userbudget_user_id_8e40cacf_fk_user_user_id` (`user_id`),
    CONSTRAINT `budgeting_userbudget_user_id_8e40cacf_fk_user_user_id` FOREIGN KEY (`user_id`) REFERENCES `user_user` (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=1 DEFAULT CHARSET=utf8
