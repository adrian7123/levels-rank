/*
  Warnings:

  - Added the required column `uuid` to the `cron` table without a default value. This is not possible if the table is not empty.

*/
-- AlterTable
ALTER TABLE `cron` ADD COLUMN `uuid` VARCHAR(191) NOT NULL;
