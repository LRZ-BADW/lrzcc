CREATE TABLE `accounting_state` (
    -- TODO: make this unsigned
    `id` int(11) NOT NULL AUTO_INCREMENT,
    `begin` datetime(6) NOT NULL,
    `end` datetime(6) DEFAULT NULL,
    PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=1 DEFAULT CHARSET=utf8
