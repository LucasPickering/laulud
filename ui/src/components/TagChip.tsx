import React from "react";
import { Chip } from "@mui/material";

interface Props {
  tag: string;
  onDelete?: () => void;
}

const TagChip: React.FC<Props> = ({ tag, onDelete }) => {
  return <Chip label={tag} color="primary" onDelete={onDelete} />;
};

export default TagChip;
