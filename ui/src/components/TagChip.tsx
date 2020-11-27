import React from "react";
import { Chip } from "@material-ui/core";

interface Props {
  className?: string;
  tag: string;
  onDelete?: () => void;
}

const TagChip: React.FC<Props> = ({ className, tag, onDelete }) => {
  return (
    <Chip
      className={className}
      label={tag}
      color="primary"
      onDelete={onDelete}
    />
  );
};

export default TagChip;
