import React, { useEffect, useState } from "react";
import { Chip, CircularProgress, InputBase, Tooltip } from "@mui/material";
import { Add as AddIcon } from "@mui/icons-material";
import { MutationStatus } from "hooks/useMutation";

interface Props extends React.ComponentProps<typeof Chip> {
  status: MutationStatus;
  addTag: (tag: string) => void;
}

const NewTagChip: React.FC<Props> = ({ status, addTag, ...rest }) => {
  const [newTagText, setNewTagText] = useState<string>("");

  // After successfully creating a tag, reset state here
  useEffect(() => {
    if (status === "success") {
      setNewTagText("");
    }
  }, [status]);

  const onSave = (): void => {
    if (newTagText) {
      addTag(newTagText);
    }
  };

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
          <form
            onSubmit={(e) => {
              e.preventDefault(); // Don't reload the page
              onSave();
            }}
          >
            <InputBase
              placeholder="New Tag"
              value={newTagText}
              onBlur={() => onSave()}
              onChange={(e) => setNewTagText(e.target.value)}
              sx={({ palette }) => ({
                width: 80,
                input: { color: palette.primary.contrastText },
              })}
            />
          </form>
        }
        clickable
      />
    </Tooltip>
  );
};

export default NewTagChip;
