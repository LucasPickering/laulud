import React from "react";
import { Button } from "@material-ui/core";

const LogOutButton: React.FC = () => {
  const logOut = () =>
    fetch("/api/logout", { method: "POST" })
      .then((response) => {
        if (response.ok) {
          // fuckin yeet em back to the home page yeehaw
          window.location.assign("/");
        }
      })
      .catch(console.error);

  return (
    <Button variant="outlined" onClick={logOut}>
      Log Out
    </Button>
  );
};

export default LogOutButton;
