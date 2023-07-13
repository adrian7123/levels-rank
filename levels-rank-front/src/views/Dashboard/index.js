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
        className="bg-primary"
        item
        xl={3}
        lg={3}
        md={1}
        sm={12}
        xs={12}
      ></Grid>
      <Grid
        className="bg-primary sm:mx-5 rounded-md"
        item
        xl={5}
        lg={5}
        md={9}
        sm={12}
        xs={12}
      >
        {data.length > 0 && <PlayersTable data={data} />}
      </Grid>
      <Grid
        className="bg-primary"
        item
        xl={3}
        lg={3}
        md={1}
        sm={12}
        xs={12}
      ></Grid>
      <Grid
        className="bg-primary"
        item
        xl={3}
        lg={1}
        md={1}
        sm={12}
        xs={12}
      ></Grid>
      <Grid className="bg-primary" item xl={5} lg={9} md={9} sm={12} xs={12}>
        {data.length > 0 && <PlayersTable data={data} />}
      </Grid>
      <Grid
        className="bg-primary"
        item
        xl={3}
        lg={1}
        md={1}
        sm={12}
        xs={12}
      ></Grid>
      <Grid
        className="bg-primary"
        item
        xl={3}
        lg={1}
        md={1}
        sm={12}
        xs={12}
      ></Grid>
      <Grid className="bg-primary" item xl={5} lg={9} md={9} sm={12} xs={12}>
        {data.length > 0 && <PlayersTable data={data} />}
      </Grid>
      <Grid
        className="bg-primary"
        item
        xl={3}
        lg={1}
        md={1}
        sm={12}
        xs={12}
      ></Grid>
    </Grid>
  );
};

export default Dashboard;
