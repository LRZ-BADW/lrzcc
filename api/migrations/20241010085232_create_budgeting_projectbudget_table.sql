CREATE TABLE `budgeting_projectbudget` (
    -- TODO: make this unsigned
    `id` int(11) NOT NULL AUTO_INCREMENT,
    `year` smallint(5) unsigned NOT NULL,
    `amount` int(10) unsigned NOT NULL,
    -- TODO: make this unsigned
    `project_id` int(11) NOT NULL,
    PRIMARY KEY (`id`),
    UNIQUE KEY `budgeting_projectbudget_year_project_id_40d8c8a7_uniq` (`year`,`project_id`),
    KEY `budgeting_projectbudget_project_id_59e782d3_fk_user_project_id` (`project_id`),
    CONSTRAINT `budgeting_projectbudget_project_id_59e782d3_fk_user_project_id` FOREIGN KEY (`project_id`) REFERENCES `user_project` (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=1 DEFAULT CHARSET=utf8
