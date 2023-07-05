import { Injectable } from '@nestjs/common';
import { PrismaService } from 'src/database/prisma.service';
import { api } from '../shared/services/api';

@Injectable()
export class PlayersService {
  constructor(private prisma: PrismaService) {}

  async getAllPlayersByServer() {
    return this.prisma.lvl_base.findMany({
      orderBy: [
        {
          value: 'desc',
        },
      ],
    });
  }

  async getAllPlayersBySteam(steamIds: string[]): Promise<any[]> {
    let res: any = { data: [] };

    try {
      res = await api.get(
        `http://api.steampowered.com/ISteamUser/GetPlayerSummaries/v0002/?key=${process.env.STEAM_WEB_KEY}&steamids=${steamIds}`,
      );
    } catch (e) {
      console.log(e);
    }

    return res.data.response.players;
  }
}
