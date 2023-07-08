import { Body, Controller, Get, Post } from '@nestjs/common';
import { PlayersService } from './players.service';
import { convertSteamID } from 'src/shared/helpers/steamIds';
import { Client, TextChannel } from 'discord.js';
import { InjectDiscordClient } from '@discord-nestjs/core';
interface IPlayerKill {
  admin_name: string;
  player_name: string;
  value: string;
}
@Controller('players')
export class PlayersController {
  constructor(
    private readonly playersService: PlayersService,
    @InjectDiscordClient()
    private readonly client: Client,
  ) {}

  @Get()
  async getAllPlayers() {
    const playersServer = await this.playersService.getAllPlayersByServer();
    const playersSteam = await this.playersService.getAllPlayersBySteam(
      playersServer.map<string>((player) => convertSteamID(player.steam)),
    );

    let id = 0;
    return playersServer.map((player) => {
      id++;

      const steam = playersSteam.find(
        (p) => p.steamid === convertSteamID(player.steam),
      );

      return { id, ...player, steam };
    });
  }

  @Post('/logs')
  async playerKill(@Body() body: IPlayerKill) {
    const channel = (await this.client.channels.fetch(
      process.env.DISCORD_LOG_CHANNEL,
    )) as TextChannel;

    console.log(body);

    const { admin_name, value, player_name } = body;

    channel.send(
      `${admin_name} ${
        value.includes('+') ? 'adicionou' : 'removeu'
      } ${value} para ${player_name}`,
    );

    return body;
  }
}
