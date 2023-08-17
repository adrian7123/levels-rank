-- CreateTable
CREATE TABLE `cron` (
    `id` VARCHAR(191) NOT NULL,
    `schedule` VARCHAR(191) NOT NULL DEFAULT '* * * * * *',
    `executed` BOOLEAN NOT NULL DEFAULT false,
    `createdAt` DATETIME(3) NOT NULL DEFAULT CURRENT_TIMESTAMP(3),
    `updatedAt` DATETIME(3) NOT NULL,
    `mixId` VARCHAR(191) NOT NULL,

    PRIMARY KEY (`id`)
) DEFAULT CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;

-- AddForeignKey
ALTER TABLE `cron` ADD CONSTRAINT `cron_mixId_fkey` FOREIGN KEY (`mixId`) REFERENCES `mix`(`id`) ON DELETE RESTRICT ON UPDATE CASCADE;
