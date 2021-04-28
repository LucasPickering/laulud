import React from "react";
import { useParams } from "react-router-dom";
import { Grid } from "@material-ui/core";
import TagList from "./TagList";
import TagDetails from "./TagDetails";
import { graphql, useLazyLoadQuery } from "react-relay";
import { TagsPageQuery } from "./__generated__/TagsPageQuery.graphql";

interface RouteParams {
  tag?: string;
}

const TagsPage: React.FC = () => {
  const params = useParams<RouteParams>();
  const tag = params.tag && decodeURIComponent(params.tag);
  const data = useLazyLoadQuery<TagsPageQuery>(
    graphql`
      query TagsPageQuery {
        tags {
          ...TagList_tagConnection
        }
      }
    `,
    {}
  );

  return (
    <Grid container spacing={2}>
      <Grid item xs={12} sm={6} md={4}>
        <TagList tagConnectionKey={data.tags} selectedTag={tag} />
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
