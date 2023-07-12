import React, { ReactNode, useEffect } from 'react';
import { useLocation } from 'react-router-dom';

interface INavigationScroll {
  children: ReactNode,
}

const NavigationScroll: React.FC<INavigationScroll> = ({ children }:INavigationScroll) => {
  const location = useLocation();
  const { pathname } = location;

  useEffect(() => {
    window.scrollTo({
      top: 0,
      left: 0,
      behavior: 'smooth'
    });
  }, [pathname]);

  return children;
};


export default NavigationScroll;
