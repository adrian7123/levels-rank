defmodule LevelsRankApiPhx.Repo do
  use Ecto.Repo,
    otp_app: :levels_rank_api_phx,
    adapter: Ecto.Adapters.MyXQL
end
