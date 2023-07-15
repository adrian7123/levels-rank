class PlayersController < ApplicationController
  def index  
    players = Player.all()

    ENV.each do |e|
      puts e
    end

    render json: players
  end

  def getSteamPlayers 
    
  end
end
