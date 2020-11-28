import React from "react";
import { Redirect } from "react-router-dom";

const HomePage: React.FC = () => {
  // We may want to put some content here in the future, but for now just redirect
  return <Redirect to="/tags" />;
};

export default HomePage;
