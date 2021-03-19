import React from "react";
import {
  List,
  ListItem,
  ListItemText,
  makeStyles,
  Paper,
  Typography,
} from "@material-ui/core";
import DataContainer from "components/generic/DataContainer";
import UnstyledLink from "components/generic/UnstyledLink";
import { useHistory } from "react-router-dom";
import TagChip from "components/TagChip";
import Link from "components/generic/Link";
import useLauludQuery from "hooks/useLauludQuery";

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

const TagList: React.FC<Props> = ({ selectedTag }) => {
  const classes = useStyles();
  const history = useHistory();
  const state = useLauludQuery(["tags"]);

  return (
    <Paper className={classes.container}>
      <DataContainer {...state}>
        {(tags) => {
          if (tags.length === 0) {
            return (
              <div className={classes.emptyState}>
                <Typography>
                  No tags yet. <Link to="/search">Search for something</Link> to
                  create your first tag.
                </Typography>
              </div>
            );
          }

          return (
            <List>
              {tags.map((tagSummary) => (
                <ListItem
                  key={tagSummary.tag}
                  button
                  selected={tagSummary.tag === selectedTag}
                  component={UnstyledLink}
                  to={{
                    ...history.location,
                    pathname: `/tags/${encodeURIComponent(tagSummary.tag)}`,
                  }}
                >
                  <ListItemText
                    primary={<TagChip tag={tagSummary.tag} />}
                    secondary={`${tagSummary.num_items} items`}
                  />
                </ListItem>
              ))}
            </List>
          );
        }}
      </DataContainer>
    </Paper>
  );
};

export default TagList;
