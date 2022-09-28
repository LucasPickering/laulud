import React, { useEffect, useState } from "react";
import { Box, Button, IconButton, Popover } from "@mui/material";
import { Add as IconAdd } from "@mui/icons-material";
import ItemSearchView from "pages/Search/ItemSearchView";
import { graphql, useQueryLoader } from "react-relay";
import { TagDetailsAddTagMutation } from "./__generated__/TagDetailsAddTagMutation.graphql";
import ErrorSnackbar from "components/generic/ErrorSnackbar";
import useMutation from "hooks/useMutation";
import type { TagDetailsItemListQuery as TagDetailsItemListQueryType } from "./__generated__/TagDetailsItemListQuery.graphql";
import TagDetailsItemListQuery from "./__generated__/TagDetailsItemListQuery.graphql";
import TagDetailsItemList from "./TagDetailsItemList";

interface Props {
  tag: string;
}

/**
 * Render detailed data about a particular tag, including a list of its items
 */
const TagDetails: React.FC<Props> = ({ tag }) => {
  const anchorEl = React.useRef<HTMLButtonElement>(null);
  const [queryRef, loadQuery] = useQueryLoader<TagDetailsItemListQueryType>(
    TagDetailsItemListQuery
  );

  // Load data
  useEffect(() => {
    loadQuery({ tag });
  }, [loadQuery, tag]);

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
            ...TagDetailsItemList_tagNode
            ...TagList_tagNode
          }
        }
      }
    }
  `);

  return (
    <>
      <Button
        ref={anchorEl}
        color="primary"
        fullWidth
        onClick={() => setIsAdding(true)}
      >
        <IconAdd />
      </Button>

      <Popover
        open={isAdding}
        anchorEl={anchorEl.current}
        onClose={() => setIsAdding(false)}
        anchorOrigin={{
          vertical: "bottom",
          horizontal: "left",
        }}
      >
        <Box width={400}>
          {/* Note: The search bar won't auto-focus in dev because of the Strict
              Mode double render, but it Works in Prod™ */}
          <ItemSearchView
            // Attach the selected tag to this item
            mapAction={(uri) => (
              <IconButton
                onClick={() =>
                  addTag({
                    variables: {
                      input: { itemUri: uri, tag },
                    },
                  })
                }
              >
                <IconAdd />
              </IconButton>
            )}
          />
        </Box>
      </Popover>

      <TagDetailsItemList queryRef={queryRef} />

      <ErrorSnackbar
        message="Error adding tag"
        status={addTagStatus}
        resetStatus={resetAddTagStatus}
      />
    </>
  );
};

export default TagDetails;
