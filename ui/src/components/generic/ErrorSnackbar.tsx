import { Snackbar } from "@material-ui/core";
import { Alert } from "@material-ui/lab";
import { MutationStatus } from "hooks/useMutation";
import React from "react";

interface Props {
  message: string;
  status: MutationStatus;
  resetStatus: () => void;
}

const ErrorSnackbar: React.FC<Props> = ({ message, status, resetStatus }) => {
  return (
    <Snackbar
      open={status === "error"}
      autoHideDuration={5000}
      onClose={resetStatus}
    >
      <Alert severity="error">{message}</Alert>
    </Snackbar>
  );
};

export default ErrorSnackbar;
