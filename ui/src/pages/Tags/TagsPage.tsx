import React from "react";
import { useParams } from "react-router-dom";
import { Grid, Paper, Typography } from "@mui/material";

import TagDetailsLoader from "./TagDetailsLoader";
import TagListLoader from "./TagListLoader";

const TagsPage: React.FC = () => {
  const params = useParams<"selectedTag">();
  const selectedTag =
    params.selectedTag && decodeURIComponent(params.selectedTag);

  return (
    <Grid container spacing={2}>
      <Grid item xs={12} sm={6} md={4}>
        <Paper>
          <TagListLoader selectedTag={selectedTag} />
        </Paper>
      </Grid>
      <Grid item xs={12} sm={6} md={8}>
        <Paper>
          {selectedTag ? (
            <TagDetailsLoader tag={selectedTag} />
          ) : (
            <Typography>Select a tag to see its tagged items.</Typography>
          )}
        </Paper>
      </Grid>
    </Grid>
  );
};

export default TagsPage;
