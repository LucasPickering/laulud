import React from "react";
import { useParams } from "react-router-dom";
import { Grid, Paper } from "@mui/material";
import { makeStyles } from "@mui/styles";
import TagDetailsLoader from "./TagDetailsLoader";
import TagListLoader from "./TagListLoader";

const useStyles = makeStyles(({ spacing }) => ({
  paperContainer: {
    padding: spacing(1),
  },
}));

const TagsPage: React.FC = () => {
  const classes = useStyles();
  const params = useParams<"selectedTag">();
  const selectedTag =
    params.selectedTag && decodeURIComponent(params.selectedTag);

  return (
    <Grid container spacing={2}>
      <Grid item xs={12} sm={6} md={4}>
        <Paper className={classes.paperContainer}>
          <TagListLoader selectedTag={selectedTag} />
        </Paper>
      </Grid>
      {selectedTag && (
        <Grid item xs={12} sm={6} md={8}>
          <Paper className={classes.paperContainer}>
            <TagDetailsLoader tag={selectedTag} />
          </Paper>
        </Grid>
      )}
    </Grid>
  );
};

export default TagsPage;
