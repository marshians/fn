import * as React from "react";

import Masonry from "@mui/lab/Masonry";
import { styled } from '@mui/material/styles';
import Paper from '@mui/material/Paper';
import Card from "@mui/material/Card";
import CardContent from "@mui/material/CardContent";
import Typography from "@mui/material/Typography";
import TextField from "@mui/material/TextField";
import Box from "@mui/material/Box";

const Item = styled(Paper)(({ theme }) => ({
  ...theme.typography.body2,
  color: theme.palette.text.secondary,
  display: "flex",
  alignItems: "center",
  justifyContent: "center",
}));

const SudokuSolver = () => {
  const [board, setBoard] = React.useState(new Array(81).fill(0));

  React.useEffect(() => {
    const listener = (event) => {
      if (event.code === "Enter" || event.code === "NumpadEnter") {
        const body = board.join("");
        event.preventDefault();
        fetch("/api/sudoku-solver", {
          method: "POST",
          body: body,
        })
          .then((res) => res.json())
          .then(
            (res) => setBoard(res.solution.split("")),
            (error) => console.log(error),
          );
      }
    };
    document.addEventListener("keydown", listener);
    return () => {
      document.removeEventListener("keydown", listener);
    };
  }, [board]);

  const updateBoard = (i) => (e) => {
    let n = [...board];
    n[i] = e.target.value;
    setBoard(n);
  };

  const digits = [
    "Digit0",
    "Digit1",
    "Digit2",
    "Digit3",
    "Digit4",
    "Digit5",
    "Digit6",
    "Digit7",
    "Digit8",
    "Digit9",
  ];

  const nextCell = (e, i) => {
    console.log(e.code, i, (i + 1) % 81);
    i = (i + 1) % 81;
    if (digits.includes(e.code)) {
      const next = document.getElementById("item-" + i);
      if (next !== null) {
        next.focus();
        next.select();
      }
    }
  };
  return (
    <React.Fragment>
      <Box sx={{ minWidth: 275 }}>
        <Card variant="outlined">
          <CardContent>
            <Typography
              sx={{ fontSize: 14 }}
              color="text.secondary"
              gutterBottom
            >
              /api/sudoku-solver
            </Typography>
            <Typography variant="h5" component="div">
              Sudoku Solver
            </Typography>
            <Typography variant="body2">
              Given an unsolved board (0 for unsolved), returns a solution, if
              possible.
            </Typography>
          </CardContent>
        </Card>
      </Box>
      <Box m="auto" sx={{ minWidth: 275, maxWidth: 500 }}>
        <Masonry columns={9} spacing={1}>
          {board.map((v, k) => (
            <Item key={k}>
              <TextField
                value={v}
                id={"item-" + k}
                onChange={updateBoard(k)}
                onKeyUp={(e) => nextCell(e, k)}
                variant="outlined"
              />
            </Item>
          ))}
        </Masonry>
      </Box>
    </React.Fragment>
  );
};

export default SudokuSolver;
