import React from "react";
import { useParams } from "react-router-dom";
import { Grid } from "@material-ui/core";
import TagList from "./TagList";
import TagDetails from "./TagDetails";
import { graphql, useLazyLoadQuery } from "react-relay";
import { TagsPageQuery } from "./__generated__/TagsPageQuery.graphql";

interface RouteParams {
  selectedTag?: string;
}

const TagsPage: React.FC = () => {
  const params = useParams<RouteParams>();
  const selectedTag =
    params.selectedTag && decodeURIComponent(params.selectedTag);
  const data = useLazyLoadQuery<TagsPageQuery>(
    graphql`
      query TagsPageQuery($selectedTag: String!, $skipTag: Boolean!) {
        tags {
          ...TagList_tagConnection
        }
        tag(tag: $selectedTag) @skip(if: $skipTag) {
          ...TagDetails_tagNode
        }
      }
    `,
    { selectedTag: selectedTag ?? "", skipTag: !selectedTag }
  );

  return (
    <Grid container spacing={2}>
      <Grid item xs={12} sm={6} md={4}>
        <TagList tagConnectionKey={data.tags} selectedTag={selectedTag} />
      </Grid>
      {data.tag && (
        <Grid item xs={12} sm={6} md={8}>
          <TagDetails tagNodeKey={data.tag} />
        </Grid>
      )}
    </Grid>
  );
};

export default TagsPage;
