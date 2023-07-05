import QotaLogo from "../assets/qota/logo-light.svg";

const Logo = ({ width, height }) => {
  return <img width={width ?? 80} height={height ?? 80} src={QotaLogo} />;
};

export default Logo;
