import { Grid } from "@mui/material";
import { useEffect, useState } from "react";
import { api } from "../../../services/api";
import { UsersTable } from "./UsersTable";

const Dashboard = () => {
  const [data, setData] = useState([]);

  async function getAllPlayers() {
    const res = await api.get("players");
    setData(res.data);
    console.log(res.data);
  }

  useEffect(() => {
    getAllPlayers();
  }, []);

  return (
    <Grid className="mt-10" container>
      <Grid item lg={2} sm={12} xs={12}></Grid>
      <Grid item lg={8} sm={12} xs={12}>
        {data.length > 0 && <UsersTable data={data} />}
      </Grid>
      <Grid item lg={2} sm={12} xs={12}></Grid>
    </Grid>
  );
};

export default Dashboard;
