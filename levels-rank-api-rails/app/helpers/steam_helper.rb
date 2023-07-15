module SteamHelper 
  def convert_steam_id(steam_id)
    if steam_id.start_with?("STEAM_")
      parts = steam_id.split(':')
      result = parts[2].to_i * 2 + 76561197960265728 + parts[1].to_i
      return result.to_s
    elsif steam_id.to_i.to_s == steam_id && steam_id.length < 16
      result = steam_id.to_i + 76561197960265728
      return result.to_s
    end
    steam_id.to_s
  end
end