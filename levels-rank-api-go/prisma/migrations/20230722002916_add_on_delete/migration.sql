-- DropForeignKey
ALTER TABLE `mix_player` DROP FOREIGN KEY `mix_player_mixId_fkey`;

-- AddForeignKey
ALTER TABLE `mix_player` ADD CONSTRAINT `mix_player_mixId_fkey` FOREIGN KEY (`mixId`) REFERENCES `mix`(`id`) ON DELETE CASCADE ON UPDATE CASCADE;
