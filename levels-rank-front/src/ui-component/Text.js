import React from "react";

const Text = ({ children }) => {
  return (
    <p className="sm:text-xs md:text-xs lg:text-xs xl:text-xs">{children}</p>
  );
};

export default Text;
