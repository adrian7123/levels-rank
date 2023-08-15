/*
  Warnings:

  - Added the required column `updatedAt` to the `mix` table without a default value. This is not possible if the table is not empty.
  - Added the required column `updatedAt` to the `mix_player` table without a default value. This is not possible if the table is not empty.

*/
-- AlterTable
ALTER TABLE `mix` ADD COLUMN `createdAt` DATETIME(3) NOT NULL DEFAULT CURRENT_TIMESTAMP(3),
    ADD COLUMN `updatedAt` DATETIME(3) NOT NULL;

-- AlterTable
ALTER TABLE `mix_player` ADD COLUMN `createdAt` DATETIME(3) NOT NULL DEFAULT CURRENT_TIMESTAMP(3),
    ADD COLUMN `updatedAt` DATETIME(3) NOT NULL;
