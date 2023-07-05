import { Controller } from '@nestjs/common';
import { DiscordBotService } from './discord_bot.service';

@Controller()
export class DiscordBotController {
  constructor(private readonly discordBotService: DiscordBotService) {}
}
