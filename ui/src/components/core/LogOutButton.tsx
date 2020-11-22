import React, { useEffect } from "react";

import { Button } from "@material-ui/core";
import { useMutation } from "react-query";
import { queryFn } from "util/queryCache";

const LogOutButton: React.FC = () => {
  const [mutate, { status }] = useMutation(() =>
    queryFn({ url: "/api/logout", method: "POST" })
  );

  useEffect(() => {
    if (status === "success") {
      // fuckin yeet em back to the home page yeehaw
      window.location.assign("/");
    }
  }, [status]);

  return (
    <Button variant="outlined" onClick={() => mutate()}>
      Log Out
    </Button>
  );
};

export default LogOutButton;
