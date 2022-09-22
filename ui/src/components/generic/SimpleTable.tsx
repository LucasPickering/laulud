import React from "react";
import { Typography, useTheme } from "@mui/material";

interface TableRow {
  label: string;
  value: React.ReactChild;
}

/**
 * A horizontal table with a simple set of labels+values. The table will have
 * two columns. The first shows labels, the second shows the corresponding values.
 * @param data The data rows
 */
const SimpleTable: React.FC<{
  data: TableRow[];
}> = ({ data }) => {
  // TODO grab theme from emotion in css prop
  const { spacing } = useTheme();
  return (
    <table css={{ borderCollapse: "collapse" }}>
      <tbody>
        {data.map(({ label, value }) => (
          <tr key={label}>
            <th css={{ textAlign: "left", paddingRight: spacing(4) }}>
              <Typography>{label}</Typography>
            </th>
            <td css={{ textAlign: "right" }}>
              <Typography>{value}</Typography>
            </td>
          </tr>
        ))}
      </tbody>
    </table>
  );
};

export default SimpleTable;
