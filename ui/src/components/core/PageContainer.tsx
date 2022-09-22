import React, { Suspense } from "react";
import { Divider, Stack, Typography } from "@mui/material";
import { makeStyles } from "@mui/styles";
import Loading from "components/Loading";
import Link from "../generic/Link";
import Header from "./Header";

const useStyles = makeStyles(({ spacing }) => ({
  pageContainer: {
    display: "flex",
    flexDirection: "column",
    alignItems: "center",
    height: "100%",
  },
  pageBody: {
    width: "100%",
    maxWidth: 1280,
    padding: spacing(2),
    paddingBottom: 0,
  },
}));

interface Props {
  showHeader?: boolean;
}

/**
 * Container for all content on the page. This is used in the root to wrap all
 * pages.
 */
const PageContainer: React.FC<Props> = ({ showHeader = true, children }) => {
  const classes = useStyles();

  return (
    <div className={classes.pageContainer}>
      {showHeader && <Header />}

      <div className={classes.pageBody}>
        <Suspense fallback={<Loading />}>{children}</Suspense>
      </div>

      <Stack
        component="footer"
        marginTop="auto"
        direction="row"
        divider={<Divider orientation="vertical" />}
        padding={2}
        spacing={1}
      >
        <Typography variant="body2">
          Created by <Link to="https://lucaspickering.me">Lucas Pickering</Link>
        </Typography>
        <Link variant="body2" to="https://github.com/LucasPickering/laulud">
          GitHub
        </Link>
      </Stack>
    </div>
  );
};

export default PageContainer;
