import React, { useState } from "react";
import { IconButton } from "@material-ui/core";
import { Add as IconAdd } from "@material-ui/icons";
import ItemList from "components/ItemList";
import ItemSearchView from "pages/Search/ItemSearchView";
import { graphql, useFragment, useMutation } from "react-relay";
import { TagDetails_tagNode$key } from "./__generated__/TagDetails_tagNode.graphql";
import { TagDetailsAddTagMutation } from "./__generated__/TagDetailsAddTagMutation.graphql";

interface Props {
  tagNodeKey: TagDetails_tagNode$key;
}

/**
 * Render pre-loaded data about a particular tag, including a list of its items
 */
const TagDetails: React.FC<Props> = ({ tagNodeKey }) => {
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
  const [addTag] = useMutation<TagDetailsAddTagMutation>(graphql`
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
      <ItemList taggedItemConnectionKey={tagNode.items} showIcons />
      {isAdding ? (
        <ItemSearchView
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
    </>
  );
};

export default TagDetails;
