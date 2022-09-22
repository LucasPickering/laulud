import React from "react";
import { List, ListItem, ListItemText, Typography } from "@mui/material";
import UnstyledLink from "components/generic/UnstyledLink";
import { useLocation } from "react-router-dom";
import TagChip from "components/TagChip";
import Link from "components/generic/Link";
import { graphql, useFragment } from "react-relay";
import { TagList_tagConnection$key } from "./__generated__/TagList_tagConnection.graphql";
import { TagList_tagNode$key } from "./__generated__/TagList_tagNode.graphql";

interface Props {
  tagConnectionKey: TagList_tagConnection$key;
  selectedTag?: string;
}

/**
 * Show a list of tags, with one of them optionally selected. The selected tag
 * will be highlighted, but no extra data rendered (that should be handled by
 * the parent). The tag list data should be pre-loaded.
 */
const TagList: React.FC<Props> = ({ tagConnectionKey, selectedTag }) => {
  const tagConnection = useFragment(
    graphql`
      fragment TagList_tagConnection on TagConnection {
        totalCount
        edges {
          node {
            id
            ...TagList_tagNode
          }
        }
      }
    `,
    tagConnectionKey
  );

  if (tagConnection.totalCount === 0) {
    return (
      <Typography padding={2}>
        No tags yet. <Link to="/search">Search for something</Link> to create
        your first tag.
      </Typography>
    );
  }

  return (
    <List>
      {tagConnection.edges.map(({ node }) => (
        <TagListItem
          key={node.id}
          tagNodeKey={node}
          selectedTag={selectedTag}
        />
      ))}
    </List>
  );
};

const TagListItem: React.FC<{
  tagNodeKey: TagList_tagNode$key;
  selectedTag?: string;
}> = ({ tagNodeKey, selectedTag }) => {
  const location = useLocation();
  const tagNode = useFragment(
    graphql`
      fragment TagList_tagNode on TagNode {
        id
        tag
        items {
          totalCount
        }
      }
    `,
    tagNodeKey
  );

  return (
    <ListItem
      key={tagNode.id}
      button
      selected={tagNode.tag === selectedTag}
      component={UnstyledLink}
      to={{
        ...location,
        pathname: `/tags/${encodeURIComponent(tagNode.tag)}`,
      }}
    >
      <ListItemText
        primary={<TagChip tag={tagNode.tag} />}
        secondary={`${tagNode.items.totalCount} items`}
      />
    </ListItem>
  );
};

export default TagList;
