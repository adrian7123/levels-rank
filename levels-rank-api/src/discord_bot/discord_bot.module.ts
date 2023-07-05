import { Module } from '@nestjs/common';
import { DiscordBotService } from './discord_bot.service';
import { DiscordBotController } from './discord_bot.controller';
import { GatewayIntentBits } from 'discord.js';
import { DiscordModule } from '@discord-nestjs/core';
import { DiscordBotGateway } from './discord_bot.gateway';

@Module({
  imports: [
    DiscordModule.forRootAsync({
      useFactory: () => ({
        token: process.env.DISCORD_TOKEN,
        commandPrefix: '%',
        discordClientOptions: {
          intents: [GatewayIntentBits.Guilds],
        },
        allowGuilds: [process.env.DISCORD_CHANNEL],
      }),
    }),
  ],
  controllers: [DiscordBotController],
  providers: [DiscordBotService, DiscordBotGateway],
})
export class DiscordBotModule {}
