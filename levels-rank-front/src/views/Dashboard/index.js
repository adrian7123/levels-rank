import { Grid } from "@mui/material";
import { useEffect, useState } from "react";
import { api } from "../../services/api";
import { PlayersTable } from "./PlayersTable";

const Dashboard = () => {
  const [data, setData] = useState(
    JSON.parse(localStorage.getItem("players")) || []
  );

  async function getAllPlayers() {
    const res = await api.get("players");

    setData([]);

    console.log(res.data);

    const data = res.data.map((player) => {
      const kd = parseFloat(
        (player.kills <= 0 ? 1 : player.kills) /
          (player.deaths <= 0 ? 1 : player.deaths)
      ).toFixed(2);
      return {
        ...player,
        kd,
        points: player.value,
        name: player.steam_data.personaname,
      };
    });

    setTimeout(() => {
      setData(data);
    }, 10);
    localStorage.setItem("players", JSON.stringify(data));
  }

  useEffect(() => {
    getAllPlayers();
  }, []);

  return (
    <Grid container className="min-h-full justify-center" gap={2}>
      <Grid
        className="bg-primary sm:mx-5 rounded-md"
        item
        xl={12}
        lg={12}
        md={12}
        sm={12}
        xs={12}
      >
        {data.length > 0 && <PlayersTable data={data} />}
      </Grid>
    </Grid>
  );
};

export default Dashboard;
