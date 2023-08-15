-- CreateTable
CREATE TABLE `mix` (
    `id` VARCHAR(191) NOT NULL,
    `date` DATETIME(3) NOT NULL DEFAULT CURRENT_TIMESTAMP(3),

    PRIMARY KEY (`id`)
) DEFAULT CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;

-- CreateTable
CREATE TABLE `mix_player` (
    `id` VARCHAR(191) NOT NULL,
    `name` VARCHAR(191) NOT NULL,
    `discord_id` VARCHAR(191) NOT NULL,
    `mixId` VARCHAR(191) NULL,

    PRIMARY KEY (`id`)
) DEFAULT CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;

-- AddForeignKey
ALTER TABLE `mix_player` ADD CONSTRAINT `mix_player_mixId_fkey` FOREIGN KEY (`mixId`) REFERENCES `mix`(`id`) ON DELETE SET NULL ON UPDATE CASCADE;
