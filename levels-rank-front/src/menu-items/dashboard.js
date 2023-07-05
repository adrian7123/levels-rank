// assets
import { IconDashboard } from "@tabler/icons";
import { ReactComponent as SteamSvg } from "../assets/qota/img/icons/custom/socials/steam-light.svg";
import { ReactComponent as DiscordSvg } from "../assets/qota/img/icons/custom/socials/discord-light.svg";

// ==============================|| DASHBOARD MENU ITEMS ||============================== //

const dashboard = {
  id: "dashboard",
  title: "Qota Community",
  type: "group",
  children: [
    {
      id: "",
      title: "Dashboard",
      type: "item",
      url: "/",
      icon: IconDashboard,
      breadcrumbs: false,
    },
    {
      id: "group",
      title: "Qota Steam Group",
      type: "item",
      url: "https://steamcommunity.com/groups/qOta-ez",
      icon: SteamSvg,
      breadcrumbs: true,
      isHttp: true,
      svg: true,
    },
    {
      id: "discord",
      title: "Discord",
      type: "item",
      url: "https://discord.gg/5f9MdSns57",
      icon: DiscordSvg,
      breadcrumbs: true,
      isHttp: true,
      svg: true,
    },
  ],
};

export default dashboard;
