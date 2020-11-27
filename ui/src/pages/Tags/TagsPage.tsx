import React from "react";
import { useParams } from "react-router-dom";
import { Grid } from "@material-ui/core";
import TagList from "./TagList";
import TagDetails from "./TagDetails";

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
          <TagDetails tag={tag} />
        </Grid>
      )}
    </Grid>
  );
};

export default TagsPage;
