import React, { useState } from "react";
import { makeStyles, Paper, IconButton, Snackbar } from "@material-ui/core";
import { Add as IconAdd } from "@material-ui/icons";
import DataContainer from "components/generic/DataContainer";
import ItemList from "components/ItemList";
import ItemSearchList from "pages/Search/ItemSearchList";
import useLauludQuery from "hooks/useLauludQuery";
import useMutationNewItemTag from "hooks/useMutationNewItemTag";
import { QueryStatus } from "react-query";
import { Alert } from "@material-ui/lab";

const useStyles = makeStyles(({ spacing }) => ({
  container: {
    padding: spacing(1),
  },
  searchBar: {
    width: "100%",
  },
  listItem: {
    flexWrap: "wrap",
  },
  listItemAvatar: {
    marginRight: spacing(2),
  },
  listItemTags: {
    flexBasis: "100%",
  },
}));

interface Props {
  tag: string;
}

const TagDetails: React.FC<Props> = ({ tag }) => {
  const classes = useStyles();

  const state = useLauludQuery(["tags", tag]);

  const [isAdding, setIsAdding] = useState<boolean>(false);
  const [addingQuery, setAddingQuery] = useState<string>("");

  const [
    createTag,
    { status: createTagStatus, reset: resetCreateTagStatus },
  ] = useMutationNewItemTag(["items", "search", addingQuery]);

  return (
    <Paper className={classes.container}>
      <DataContainer {...state}>
        {(tagDetails) => (
          <>
            <ItemList items={tagDetails.items} showIcons />
            {isAdding ? (
              <ItemSearchList
                searchQuery={addingQuery}
                setSearchQuery={setAddingQuery}
                // Attach the selected take to this item
                mapAction={(item) => (
                  <IconButton
                    onClick={() => createTag({ uri: item.data.uri, tag })}
                  >
                    <IconAdd />
                  </IconButton>
                )}
              />
            ) : (
              <IconButton onClick={() => setIsAdding(true)}>
                <IconAdd />
              </IconButton>
            )}

            <Snackbar
              open={createTagStatus === QueryStatus.Error}
              autoHideDuration={5000}
              onClose={() => resetCreateTagStatus()}
            >
              <Alert severity="error">Error creating tag</Alert>
            </Snackbar>
          </>
        )}
      </DataContainer>
    </Paper>
  );
};

export default TagDetails;
