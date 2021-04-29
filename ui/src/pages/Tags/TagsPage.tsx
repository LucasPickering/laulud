import React from "react";
import { useParams } from "react-router-dom";
import { Grid } from "@material-ui/core";
import TagList from "./TagList";
import { graphql, useLazyLoadQuery } from "react-relay";
import { TagsPageQuery } from "./__generated__/TagsPageQuery.graphql";
import TagDetailsView from "./TagDetailsView";

interface RouteParams {
  selectedTag?: string;
}

const TagsPage: React.FC = () => {
  const params = useParams<RouteParams>();
  const selectedTag =
    params.selectedTag && decodeURIComponent(params.selectedTag);

  const tagsData = useLazyLoadQuery<TagsPageQuery>(
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
        <TagList tagConnectionKey={tagsData.tags} selectedTag={selectedTag} />
      </Grid>
      {selectedTag && (
        <Grid item xs={12} sm={6} md={8}>
          <TagDetailsView tag={selectedTag} />
        </Grid>
      )}
    </Grid>
  );
};

export default TagsPage;
