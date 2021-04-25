import React from "react";
import {
  List,
  ListItem,
  ListItemText,
  makeStyles,
  Paper,
  Typography,
} from "@material-ui/core";
import UnstyledLink from "components/generic/UnstyledLink";
import { useHistory } from "react-router-dom";
import TagChip from "components/TagChip";
import Link from "components/generic/Link";
import { useLazyLoadQuery } from "react-relay";
import { TagListQuery } from "./__generated__/TagListQuery.graphql";

const useStyles = makeStyles(({ spacing }) => ({
  container: {
    padding: spacing(1),
  },
  emptyState: {
    padding: spacing(2),
  },
}));

interface Props {
  selectedTag?: string;
}

/**
 * Show a list of tags, with one of them optionally selected. The selected tag
 * will be highlighted, but no extra data rendered (that should be handled by
 * the parent).
 */
const TagList: React.FC<Props> = ({ selectedTag }) => {
  const classes = useStyles();
  const history = useHistory();

  const tags = useLazyLoadQuery<TagListQuery>(
    graphql`
      query TagListQuery {
        tags {
          totalCount
          edges {
            node {
              id
              tag
              items {
                totalCount
              }
            }
          }
        }
      }
    `,
    {}
  );

  return (
    <Paper className={classes.container}>
      {tags.totalCount > 0 ? (
        <List>
          {tags.edges.map(({ node: tagNode }) => (
            <ListItem
              key={node.id}
              button
              selected={tagNode.tag === selectedTag}
              component={UnstyledLink}
              to={{
                ...history.location,
                pathname: `/tags/${encodeURIComponent(tagNode.tag)}`,
              }}
            >
              <ListItemText
                primary={<TagChip tag={tagNode.tag} />}
                secondary={`${node.items.totalCount} items`}
              />
            </ListItem>
          ))}
        </List>
      ) : (
        <div className={classes.emptyState}>
          <Typography>
            No tags yet. <Link to="/search">Search for something</Link> to
            create your first tag.
          </Typography>
        </div>
      )}
    </Paper>
  );
};

export default TagList;
