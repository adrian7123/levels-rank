import { TableCell, TableHead, TableRow, TableSortLabel } from "@mui/material";
import { Box } from "@mui/system";
import PropTypes from "prop-types";
import { visuallyHidden } from "@mui/utils";
import Text from "../../../ui-component/Text";
function EnhancedTableHead(props) {
  const {
    onSelectAllClick,
    order,
    orderBy,
    numSelected,
    rowCount,
    onRequestSort,
  } = props;
  const createSortHandler = (property) => (event) => {
    onRequestSort(event, property);
  };

  const headCells = [
    {
      id: "id",
      numeric: false,
      disablePadding: true,
      label: "#",
    },
    {
      id: "name",
      numeric: false,
      disablePadding: true,
      label: "PLAYER",
    },
    {
      id: "points",
      numeric: false,
      disablePadding: true,
      label: "POINTS",
    },
    {
      id: "kills",
      numeric: false,
      disablePadding: true,
      label: "KILLS",
    },
    {
      id: "deaths",
      numeric: false,
      disablePadding: true,
      label: "DEATHS",
    },
    {
      id: "rank",
      numeric: false,
      disablePadding: true,
      label: "RANK",
    },
    {
      id: "kd",
      numeric: false,
      disablePadding: true,
      label: "K/D",
    },
  ];

  return (
    <TableHead>
      <TableRow>
        <TableCell padding="checkbox">
          <div className="h-14"></div>
        </TableCell>
        {headCells.map((headCell) => (
          <TableCell
            key={headCell.id}
            align={headCell.numeric ? "right" : "left"}
            padding={headCell.disablePadding ? "none" : "normal"}
            sortDirection={orderBy === headCell.id ? order : false}
          >
            <TableSortLabel
              active={orderBy === headCell.id}
              direction={orderBy === headCell.id ? order : "asc"}
              onClick={createSortHandler(headCell.id)}
            >
              <Text>{headCell.label}</Text>
              {orderBy === headCell.id ? (
                <Box component="span" sx={visuallyHidden}>
                  {order === "desc" ? "sorted descending" : "sorted ascending"}
                </Box>
              ) : null}
            </TableSortLabel>
          </TableCell>
        ))}
      </TableRow>
    </TableHead>
  );
}

EnhancedTableHead.propTypes = {
  numSelected: PropTypes.number.isRequired,
  onRequestSort: PropTypes.func.isRequired,
  onSelectAllClick: PropTypes.func,
  order: PropTypes.oneOf(["asc", "desc"]).isRequired,
  orderBy: PropTypes.string.isRequired,
  rowCount: PropTypes.number.isRequired,
};

export { EnhancedTableHead };
