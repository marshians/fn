import * as React from "react";

import Card from "@mui/material/Card";
import CardActions from "@mui/material/CardActions";
import CardContent from "@mui/material/CardContent";
import Button from "@mui/material/Button";
import Typography from "@mui/material/Typography";

import { Link } from "react-router-dom";

const ff = [
  { name: "sudoku", link: "/sudoku-solver", description: "sudoku solver" },
  {
    name: "letters",
    link: "/letters-to-words",
    description: "generate words from a list of letters",
  },
];

const cards = ff.map((f) => {
  return (
    <Card sx={{ minWidth: 275 }} key={f.name}>
      <CardContent>
        <Typography variant="h5" component="div">
          {f.name}
        </Typography>
        <Typography variant="body2">{f.description}</Typography>
      </CardContent>
      <CardActions>
        <Button
          component={Link}
          to={f.link}
          variant="contained"
          color="success"
          size="small"
          sx={{ marginLeft: "auto" }}
        >
          Go
        </Button>
      </CardActions>
    </Card>
  );
});

const Functions = () => {
  return <div>{cards}</div>;
};

export default Functions;
