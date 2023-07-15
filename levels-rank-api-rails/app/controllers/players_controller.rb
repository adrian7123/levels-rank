class PlayersController < ApplicationController
  include SteamHelper

  def index  
    players_model = Player.order("value DESC")
    players_ids = []
    steam_players = []
    players = []

    players_model.each do |player|
      players_ids << convert_steam_id(player.steam)
      players << JSON.parse(player.to_json)
    end

    steam_players = JSON.parse(getSteamPlayers(players_ids))['response']['players']
    
    id = 0

    steam_players.each do |steam_player| 
      player_index = players.find_index { |p|convert_steam_id(p['steam'])==steam_player['steamid']}
      players[player_index]['steam_data'] = steam_player
      players[player_index]['id'] = id+1
    end

    render json: players
  end

  def getSteamPlayers(players_ids)
    steam_web_key = ENV.fetch('RAILS_STEAM_WEB_KEY')

    uri = URI("http://api.steampowered.com/ISteamUser/GetPlayerSummaries/v0002/?key=#{steam_web_key}&steamids=#{players_ids}")
    res = Net::HTTP.get_response(uri)
    res.body
  end
end
