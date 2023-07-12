import { createBrowserRouter } from "react-router-dom";
import App from "../App";
import Home from "../pages/Home";

const MainRoutes = createBrowserRouter([
  {
    path: "/",
    element: <Home />,
  },
]);

export default MainRoutes;
