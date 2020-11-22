import React from "react";
import { Redirect } from "react-router-dom";

const HomePage: React.FC = () => {
  // We may want to put some center here in the future, but for now just redirect
  return <Redirect to="/tracks" />;
};

export default HomePage;
