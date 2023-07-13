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

    setTimeout(() => {
      setData(res.data);
    }, 10);
    localStorage.setItem("players", JSON.stringify(res.data));
  }

  useEffect(() => {
    getAllPlayers();
  }, []);

  return (
    <Grid className="mt-10" container>
      <Grid item lg={2} sm={12} xs={12}></Grid>
      <Grid item lg={8} sm={12} xs={12}>
        {data.length > 0 && <PlayersTable data={data} />}
      </Grid>
      <Grid item lg={2} sm={12} xs={12}></Grid>
    </Grid>
  );
};

export default Dashboard;
