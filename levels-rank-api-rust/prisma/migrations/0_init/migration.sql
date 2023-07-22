-- CreateTable
CREATE TABLE `lvl_base` (
    `steam` VARCHAR(22) NOT NULL,
    `name` VARCHAR(128) NOT NULL DEFAULT '',
    `value` INTEGER NOT NULL DEFAULT 0,
    `rank` INTEGER NOT NULL DEFAULT 0,
    `kills` INTEGER NOT NULL DEFAULT 0,
    `deaths` INTEGER NOT NULL DEFAULT 0,
    `shoots` INTEGER NOT NULL DEFAULT 0,
    `hits` INTEGER NOT NULL DEFAULT 0,
    `headshots` INTEGER NOT NULL DEFAULT 0,
    `assists` INTEGER NOT NULL DEFAULT 0,
    `round_win` INTEGER NOT NULL DEFAULT 0,
    `round_lose` INTEGER NOT NULL DEFAULT 0,
    `playtime` INTEGER NOT NULL DEFAULT 0,
    `lastconnect` INTEGER NOT NULL DEFAULT 0,

    PRIMARY KEY (`steam`)
) DEFAULT CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;

-- CreateTable
CREATE TABLE `lr_web_attendance` (
    `id` INTEGER UNSIGNED NOT NULL AUTO_INCREMENT,
    `date` VARCHAR(10) NOT NULL,
    `visits` INTEGER NOT NULL,

    PRIMARY KEY (`id`)
) DEFAULT CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;

-- CreateTable
CREATE TABLE `lr_web_cookie_tokens` (
    `id` INTEGER NOT NULL AUTO_INCREMENT,
    `steam` VARCHAR(255) NOT NULL DEFAULT '0',
    `cookie_expire` VARCHAR(255) NOT NULL DEFAULT '0',
    `cookie_token` VARCHAR(255) NOT NULL DEFAULT '0',

    PRIMARY KEY (`id`)
) DEFAULT CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;

-- CreateTable
CREATE TABLE `lr_web_notifications` (
    `id` INTEGER UNSIGNED NOT NULL AUTO_INCREMENT,
    `steam` VARCHAR(128) NOT NULL,
    `text` VARCHAR(256) NOT NULL,
    `values_insert` VARCHAR(512) NOT NULL,
    `url` VARCHAR(128) NOT NULL,
    `icon` VARCHAR(64) NOT NULL,
    `seen` INTEGER NOT NULL,
    `status` INTEGER NOT NULL,
    `date` TIMESTAMP(0) NOT NULL DEFAULT CURRENT_TIMESTAMP(0),

    PRIMARY KEY (`id`)
) DEFAULT CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;

-- CreateTable
CREATE TABLE `lr_web_online` (
    `id` INTEGER UNSIGNED NOT NULL AUTO_INCREMENT,
    `user` VARCHAR(128) NOT NULL,
    `ip` VARCHAR(128) NOT NULL,
    `time` TIMESTAMP(0) NOT NULL DEFAULT CURRENT_TIMESTAMP(0),

    PRIMARY KEY (`id`)
) DEFAULT CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;

-- CreateTable
CREATE TABLE `lvl_base_hits` (
    `SteamID` VARCHAR(32) NOT NULL DEFAULT '',
    `DmgHealth` INTEGER NOT NULL DEFAULT 0,
    `DmgArmor` INTEGER NOT NULL DEFAULT 0,
    `Head` INTEGER NOT NULL DEFAULT 0,
    `Chest` INTEGER NOT NULL DEFAULT 0,
    `Belly` INTEGER NOT NULL DEFAULT 0,
    `LeftArm` INTEGER NOT NULL DEFAULT 0,
    `RightArm` INTEGER NOT NULL DEFAULT 0,
    `LeftLeg` INTEGER NOT NULL DEFAULT 0,
    `RightLeg` INTEGER NOT NULL DEFAULT 0,
    `Neak` INTEGER NOT NULL DEFAULT 0,

    PRIMARY KEY (`SteamID`)
) DEFAULT CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;

-- CreateTable
CREATE TABLE `lvl_base_maps` (
    `steam` VARCHAR(32) NOT NULL DEFAULT '',
    `name_map` VARCHAR(128) NOT NULL DEFAULT '',
    `countplays` INTEGER NOT NULL DEFAULT 0,
    `kills` INTEGER NOT NULL DEFAULT 0,
    `deaths` INTEGER NOT NULL DEFAULT 0,
    `rounds_overall` INTEGER NOT NULL DEFAULT 0,
    `rounds_ct` INTEGER NOT NULL DEFAULT 0,
    `rounds_t` INTEGER NOT NULL DEFAULT 0,
    `bomb_planted` INTEGER NOT NULL DEFAULT 0,
    `bomb_defused` INTEGER NOT NULL DEFAULT 0,
    `hostage_rescued` INTEGER NOT NULL DEFAULT 0,
    `hostage_killed` INTEGER NOT NULL DEFAULT 0,
    `playtime` INTEGER NOT NULL DEFAULT 0,

    PRIMARY KEY (`steam`, `name_map`)
) DEFAULT CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;

-- CreateTable
CREATE TABLE `lvl_base_weapons` (
    `steam` VARCHAR(32) NOT NULL DEFAULT '',
    `classname` VARCHAR(64) NOT NULL DEFAULT '',
    `kills` INTEGER NOT NULL DEFAULT 0,

    PRIMARY KEY (`steam`, `classname`)
) DEFAULT CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;

-- CreateTable
CREATE TABLE `lvl_web_admins` (
    `steamid` VARCHAR(32) NOT NULL,
    `user` VARCHAR(32) NOT NULL,
    `password` VARCHAR(64) NOT NULL,
    `ip` VARCHAR(16) NOT NULL,
    `group` VARCHAR(11) NOT NULL,
    `flags` VARCHAR(32) NOT NULL,
    `access` INTEGER NOT NULL,

    PRIMARY KEY (`steamid`)
) DEFAULT CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;

-- CreateTable
CREATE TABLE `lvl_web_servers` (
    `id` INTEGER UNSIGNED NOT NULL AUTO_INCREMENT,
    `ip` VARCHAR(64) NOT NULL,
    `fakeip` VARCHAR(64) NOT NULL,
    `name` VARCHAR(64) NOT NULL,
    `name_custom` VARCHAR(128) NOT NULL,
    `rcon` VARCHAR(64) NOT NULL,
    `server_stats` VARCHAR(64) NOT NULL,
    `server_vip` VARCHAR(64) NOT NULL,
    `server_vip_id` INTEGER NOT NULL,
    `server_sb` VARCHAR(64) NOT NULL,
    `server_shop` VARCHAR(64) NOT NULL,
    `server_warnsystem` VARCHAR(64) NOT NULL,
    `server_lk` VARCHAR(64) NOT NULL,

    PRIMARY KEY (`id`)
) DEFAULT CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;

-- CreateTable
CREATE TABLE `lvl_web_settings` (
    `name` VARCHAR(64) NOT NULL,
    `value` VARCHAR(256) NOT NULL,

    PRIMARY KEY (`name`)
) DEFAULT CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;

