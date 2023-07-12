import { RouterProvider } from "react-router-dom";
import MainRoutes from "./MainRoutes";

import React from "react";

const Routes: React.FC = () => {
  return <RouterProvider router={MainRoutes} />;
};

export default Routes;
