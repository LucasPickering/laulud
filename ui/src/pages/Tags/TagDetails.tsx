import React, { useState } from "react";
import { Button, IconButton } from "@mui/material";
import { makeStyles } from "@mui/styles";
import { Add as IconAdd } from "@mui/icons-material";
import ItemList from "components/ItemList";
import ItemSearchView from "pages/Search/ItemSearchView";
import { graphql, useFragment } from "react-relay";
import { TagDetails_tagNode$key } from "./__generated__/TagDetails_tagNode.graphql";
import { TagDetailsAddTagMutation } from "./__generated__/TagDetailsAddTagMutation.graphql";
import ErrorSnackbar from "components/generic/ErrorSnackbar";
import useMutation from "hooks/useMutation";

const useStyles = makeStyles({
  addButton: {
    width: "100%",
  },
});

interface Props {
  tagNodeKey: TagDetails_tagNode$key;
}

/**
 * Render pre-loaded data about a particular tag, including a list of its items
 */
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
  const {
    commit: addTag,
    status: addTagStatus,
    resetStatus: resetAddTagStatus,
  } = useMutation<TagDetailsAddTagMutation>(graphql`
    mutation TagDetailsAddTagMutation($input: AddTagInput!) {
      addTag(input: $input) {
        # Grab this data so relay can update it in the store
        itemEdge {
          node {
            ...TagChips_taggedItemNode
          }
        }
        tagEdge {
          node {
            ...TagDetails_tagNode
            ...TagList_tagNode
          }
        }
      }
    }
  `);

  return (
    <>
      {isAdding ? (
        <ItemSearchView
          // Attach the selected tag to this item
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
        <Button className={classes.addButton} onClick={() => setIsAdding(true)}>
          <IconAdd />
        </Button>
      )}

      <ItemList taggedItemConnectionKey={tagNode.items} showIcons />

      <ErrorSnackbar
        message="Error adding tag"
        status={addTagStatus}
        resetStatus={resetAddTagStatus}
      />
    </>
  );
};

export default TagDetails;
