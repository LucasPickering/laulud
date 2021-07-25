import React from "react";
import { makeStyles, Typography } from "@material-ui/core";

const useLocalStyles = makeStyles(({ spacing }) => ({
  simpleTable: {
    borderCollapse: "collapse",
  },
  simpleTableLabel: {
    textAlign: "left",
    paddingRight: spacing(4),
  },
  simpleTableValue: {
    textAlign: "right",
  },
}));

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
  const localClasses = useLocalStyles();

  return (
    <table className={localClasses.simpleTable}>
      <tbody>
        {data.map(({ label, value }) => (
          <tr key={label}>
            <th className={localClasses.simpleTableLabel}>
              <Typography>{label}</Typography>
            </th>
            <td className={localClasses.simpleTableValue}>
              <Typography>{value}</Typography>
            </td>
          </tr>
        ))}
      </tbody>
    </table>
  );
};

export default SimpleTable;
