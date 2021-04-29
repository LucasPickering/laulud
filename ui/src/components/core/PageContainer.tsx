import React, { Suspense } from "react";
import { makeStyles, Typography } from "@material-ui/core";
import Loading from "components/Loading";
import Link from "../generic/Link";
import Header from "./Header";

const useStyles = makeStyles(({ palette, spacing }) => ({
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
  pageFooter: {
    marginTop: "auto",
    padding: spacing(2),
    display: "flex",
    justifyContent: "center",
    "& > *": {
      padding: `0px ${spacing(1)}px`,
    },
    "& > * + *": {
      borderLeftWidth: 1,
      borderLeftStyle: "solid",
      borderLeftColor: palette.divider,
    },
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

      <footer className={classes.pageFooter}>
        <Typography variant="body2">
          Created by <Link to="https://lucaspickering.me">Lucas Pickering</Link>
        </Typography>
        <Link to="https://github.com/LucasPickering/laulud">GitHub</Link>
      </footer>
    </div>
  );
};

export default PageContainer;
