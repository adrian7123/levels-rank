/*
  Warnings:

  - You are about to drop the `cron` table. If the table is not empty, all the data it contains will be lost.

*/
-- DropForeignKey
ALTER TABLE `cron` DROP FOREIGN KEY `cron_mixId_fkey`;

-- DropTable
DROP TABLE `cron`;

-- CreateTable
CREATE TABLE `mix_schedule` (
    `id` VARCHAR(191) NOT NULL,
    `uuid` VARCHAR(191) NOT NULL,
    `schedule` VARCHAR(191) NOT NULL DEFAULT '* * * * * *',
    `executed` BOOLEAN NOT NULL DEFAULT false,
    `createdAt` DATETIME(3) NOT NULL DEFAULT CURRENT_TIMESTAMP(3),
    `updatedAt` DATETIME(3) NOT NULL,
    `mixId` VARCHAR(191) NOT NULL,

    PRIMARY KEY (`id`)
) DEFAULT CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;

-- AddForeignKey
ALTER TABLE `mix_schedule` ADD CONSTRAINT `mix_schedule_mixId_fkey` FOREIGN KEY (`mixId`) REFERENCES `mix`(`id`) ON DELETE CASCADE ON UPDATE CASCADE;
