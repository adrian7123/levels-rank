import { Module } from '@nestjs/common';
import { AppController } from './app.controller';
import { AppService } from './app.service';
import { PlayersModule } from './players/players.module';
import { DiscordBotModule } from './discord_bot/discord_bot.module';

@Module({
  imports: [PlayersModule, DiscordBotModule],
  controllers: [AppController],
  providers: [AppService],
})
export class AppModule {}
