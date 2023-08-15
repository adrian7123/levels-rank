package main

import (
	"context"
	"encoding/json"
	"fmt"

	"levels-rank-api-go/prisma/db"
)
 
func main() {
  if err := run(); err != nil {
    panic(err)
  }
}
 
func run() error {
  client := db.NewClient()
  if err := client.Prisma.Connect(); err != nil {
    return err
  }
 
  defer func() {
    if err := client.Prisma.Disconnect(); err != nil {
      panic(err)
    }
  }()
 
  ctx := context.Background()
 
  players,err := client.LvlBase.FindMany().Exec(ctx)
  if err != nil {
    return err
  }
 
  result, _ := json.MarshalIndent(players, "", "  ")
  fmt.Printf("post: %s\n", result)
 
  fmt.Printf("The posts's description is: %s\n", result)
 
  return nil
}