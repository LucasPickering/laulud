import React from "react";
import { useParams } from "react-router-dom";
import { Grid } from "@material-ui/core";
import TagList from "./TagList";

interface RouteParams {
  tag?: string;
}

const TagsPage: React.FC = () => {
  const { tag } = useParams<RouteParams>();

  return (
    <Grid container spacing={2}>
      <Grid item xs={12} sm={6} md={4}>
        <TagList selectedTag={tag} />
      </Grid>
      {tag && (
        <Grid item xs={12} sm={6} md={8}>
          content
        </Grid>
      )}
    </Grid>
  );
};

export default TagsPage;
