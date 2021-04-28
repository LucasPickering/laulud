import React, { useState } from "react";
import { makeStyles, Paper, IconButton, Snackbar } from "@material-ui/core";
import { Add as IconAdd } from "@material-ui/icons";
import ItemList from "components/ItemList";
import ItemSearchList from "pages/Search/ItemSearchList";
import { Alert } from "@material-ui/lab";
import { graphql, useFragment, useMutation } from "react-relay";
import { TagDetails_tagNode$key } from "./__generated__/TagDetails_tagNode.graphql";
import { TagDetailsAddTagMutation } from "./__generated__/TagDetailsAddTagMutation.graphql";

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
  tagNodeKey: TagDetails_tagNode$key;
}

const TagDetails: React.FC<Props> = ({ tagNodeKey }) => {
  const classes = useStyles();

  const tagNode = useFragment(
    graphql`
      fragment TagDetails_tagNode on TagNode {
        tag
        items {
          ...ItemList_taggedItemConnection
        }
      }
    `,
    tagNodeKey
  );

  // Stuff to allow adding more items to this tag
  const [isAdding, setIsAdding] = useState<boolean>(false);
  const [addingQuery, setAddingQuery] = useState<string>("");
  const [addTag, isAddInFlight] = useMutation<TagDetailsAddTagMutation>(graphql`
    mutation TagDetailsAddTagMutation($input: AddTagInput!) {
      addTag(input: $input) {
        item {
          ...TagChips_taggedItemNode
        }
      }
    }
  `);

  return (
    <Paper className={classes.container}>
      <ItemList taggedItemConnectionKey={tagNode.items} showIcons />
      {isAdding ? (
        <ItemSearchList
          searchQuery={addingQuery}
          setSearchQuery={setAddingQuery}
          // Attach the selected take to this item
          mapAction={(uri) => (
            <IconButton
              onClick={() =>
                addTag({
                  variables: {
                    input: { itemUri: uri, tag: tagNode.tag },
                  },
                })
              }
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

      {/* TODO enable snack bar */}
      {/* <Snackbar
        open={createTagStatus === QueryStatus.Error}
        autoHideDuration={5000}
        onClose={() => resetCreateTagStatus()}
      >
        <Alert severity="error">Error creating tag</Alert>
      </Snackbar> */}
    </Paper>
  );
};

export default TagDetails;
