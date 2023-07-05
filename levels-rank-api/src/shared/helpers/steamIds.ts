export function convertSteamID(steamId: string): string {
  if (steamId.startsWith('STEAM_')) {
    const parts = steamId.split(':');
    const result =
      BigInt(parts[2]) * BigInt(2) +
      BigInt('76561197960265728') +
      BigInt(parts[1]);
    return result.toString();
  } else if (Number.isInteger(Number(steamId)) && steamId.length < 16) {
    const result = BigInt(steamId) + BigInt('76561197960265728');
    return result.toString();
  } else {
    return steamId;
  }
}
