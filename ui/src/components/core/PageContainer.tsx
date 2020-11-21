import { makeStyles, Typography } from "@material-ui/core";
import React from "react";

import Link from "../generic/Link";
import Header from "./Header";

const useLocalStyles = makeStyles(({ palette, spacing }) => ({
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
      padding: `0px ${spacing(1)}`,
    },
    "& > * + *": {
      borderLeftWidth: 1,
      borderLeftStyle: "solid",
      borderLeftColor: palette.divider,
    },
  },
}));

/**
 * Container for all content on the page. This is used in the root to wrap all
 * pages.
 */
const PageContainer: React.FC = ({ children }) => {
  const localClasses = useLocalStyles();

  return (
    <div className={localClasses.pageContainer}>
      <Header />

      <div className={localClasses.pageBody}>{children}</div>

      <footer className={localClasses.pageFooter}>
        <Typography variant="body2">
          Created by <Link to="https://lucaspickering.me">Lucas Pickering</Link>
        </Typography>
        <Link to="https://github.com/LucasPickering/laulud">GitHub</Link>
      </footer>
    </div>
  );
};

export default PageContainer;
