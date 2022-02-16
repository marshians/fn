import * as React from "react";

import AppBar from "@mui/material/AppBar";
import Toolbar from "@mui/material/Toolbar";
import Typography from "@mui/material/Typography";
import IconButton from "@mui/material/IconButton";

import HomeIcon from "@mui/icons-material/Home";

import { BrowserRouter as Router, Switch, Route, Link } from "react-router-dom";

import Functions from "./Functions";
import SudokuSolver from "./SudokuSolver";
import LettersToWords from "./LettersToWords";

const App = () => {
  return (
    <Router>
      <div>
        <AppBar position="sticky">
          <Toolbar>
            <Typography variant="h6" color="inherit" component="div">
              Fn
            </Typography>
            <IconButton
              sx={{ ml: "2rem" }}
              edge="start"
              color="inherit"
              aria-label="home"
              component={Link}
              to="/"
            >
              <HomeIcon />
            </IconButton>
          </Toolbar>
        </AppBar>

        <Switch>
          <Route path="/sudoku-solver">
            <SudokuSolver />
          </Route>
          <Route path="/letters-to-words">
            <LettersToWords />
          </Route>
          <Route path="/">
            <Functions />
          </Route>
        </Switch>
      </div>
    </Router>
  );
};

export default App;
