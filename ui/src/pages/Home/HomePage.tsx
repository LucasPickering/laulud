import React from "react";
import { Navigate } from "react-router-dom";

const HomePage: React.FC = () => {
  // We may want to put some content here in the future, but for now just redirect
  return <Navigate replace to="/tags" />;
};

export default HomePage;
