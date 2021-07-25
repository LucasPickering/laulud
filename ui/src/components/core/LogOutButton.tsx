import React from "react";
import { Button } from "@material-ui/core";

const LogOutButton: React.FC = () => (
  <Button
    variant="outlined"
    onClick={() =>
      fetch("/api/logout", { method: "POST" })
        .then((response) => {
          if (response.ok) {
            // fuckin yeet em back to the home page yeehaw
            window.location.assign("/");
          }
        })
        // eslint-disable-next-line no-console
        .catch(console.error)
    }
  >
    Log Out
  </Button>
);

export default LogOutButton;
