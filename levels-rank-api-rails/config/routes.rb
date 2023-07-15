Rails.application.routes.draw do
  root "application#index"
  get "/players", to: "players#index"
  # Define your application routes per the DSL in https://guides.rubyonrails.org/routing.html

  # Defines the root path route ("/")
  # root "articles#index"
end
