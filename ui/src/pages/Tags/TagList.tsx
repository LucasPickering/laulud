import React from "react";
import {
  Chip,
  List,
  ListItem,
  ListItemText,
  makeStyles,
  Paper,
} from "@material-ui/core";
import DataContainer from "components/generic/DataContainer";
import { useQuery } from "react-query";
import UnstyledLink from "components/generic/UnstyledLink";
import { queryFn } from "util/queryCache";
import { useHistory } from "react-router-dom";

const useStyles = makeStyles(({ spacing }) => ({
  container: {
    padding: spacing(1),
  },
}));

interface Props {
  selectedTag?: string;
}

const TagList: React.FC<Props> = ({ selectedTag }) => {
  const classes = useStyles();
  const history = useHistory();
  const state = useQuery("tags", () => queryFn({ url: "/api/tags" }));

  return (
    <Paper className={classes.container}>
      <DataContainer {...state}>
        {(tags) => (
          <List>
            {tags.map((tag) => (
              <ListItem
                key={tag.tag}
                button
                selected={tag.tag === selectedTag}
                component={UnstyledLink}
                to={{
                  ...history.location,
                  pathname: `/tags/${tag.tag}`,
                }}
              >
                <ListItemText
                  primary={<Chip label={tag.tag} color="primary" />}
                  // secondary={}
                />
              </ListItem>
            ))}
          </List>
        )}
      </DataContainer>
    </Paper>
  );
};

export default TagList;
