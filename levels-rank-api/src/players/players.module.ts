import { Module } from '@nestjs/common';
import { PlayersService } from './players.service';
import { PlayersController } from './players.controller';
import { PrismaService } from 'src/database/prisma.service';
import { DiscordModule } from '@discord-nestjs/core';
import { GatewayIntentBits } from 'discord.js';

@Module({
  imports: [
    DiscordModule.forRootAsync({
      useFactory: () => ({
        token: process.env.DISCORD_TOKEN,
        commandPrefix: '%',
        discordClientOptions: {
          intents: [
            GatewayIntentBits.Guilds,
            GatewayIntentBits.MessageContent,
            GatewayIntentBits.DirectMessages,
            GatewayIntentBits.GuildMessages,
          ],
        },
        allowGuilds: [process.env.DISCORD_CHANNEL],
      }),
    }),
  ],
  controllers: [PlayersController],
  providers: [PlayersService, PrismaService],
})
export class PlayersModule {}
