import React, { useEffect, useState } from "react";
import { Chip, CircularProgress, InputBase, Tooltip } from "@mui/material";
import { Add as AddIcon } from "@mui/icons-material";
import { MutationStatus } from "hooks/useMutation";

interface Props extends React.ComponentProps<typeof Chip> {
  status: MutationStatus;
  addTag: (tag: string) => void;
}

const NewTagChip: React.FC<Props> = ({ status, addTag, ...rest }) => {
  const [isEditing, setIsEditing] = useState<boolean>(false);
  const [newTagText, setNewTagText] = useState<string>("");

  // After successfully creating a tag, reset state here
  useEffect(() => {
    if (status === "success") {
      setIsEditing(false);
      setNewTagText("");
    }
  }, [status]);

  return (
    <Tooltip title="Add Tag">
      <Chip
        {...rest}
        icon={
          status === "loading" ? (
            <CircularProgress color="secondary" size={24} />
          ) : (
            <AddIcon />
          )
        }
        label={
          isEditing ? (
            <form
              onSubmit={(e) => {
                e.preventDefault();
                addTag(newTagText);
                setIsEditing(false);
              }}
            >
              <InputBase
                autoFocus
                value={newTagText}
                onBlur={() => setIsEditing(false)}
                onChange={(e) => setNewTagText(e.target.value)}
              />
            </form>
          ) : (
            newTagText || null
          )
        }
        clickable
        onClick={() => setIsEditing(true)}
      />
    </Tooltip>
  );
};

export default NewTagChip;
