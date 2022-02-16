import * as React from "react";

import { styled } from "@mui/material/styles";
import Masonry from "@mui/lab/Masonry";
import Paper from "@mui/material/Paper";
import Card from "@mui/material/Card";
import CardContent from "@mui/material/CardContent";
import Typography from "@mui/material/Typography";
import MenuItem from "@mui/material/MenuItem";
import TextField from "@mui/material/TextField";
import Box from "@mui/material/Box";

const Item = styled(Paper)(({ theme }) => ({
  ...theme.typography.body2,
  color: theme.palette.text.secondary,
  display: "flex",
  alignItems: "center",
  justifyContent: "center",
}));

const LettersToWords = () => {
  const [letters, setLetters] = React.useState("");
  const [min, setMin] = React.useState(3);
  const [words, setWords] = React.useState([]);

  React.useEffect(() => {
    const listener = (event) => {
      if (event.code === "Enter" || event.code === "NumpadEnter") {
        event.preventDefault();
        const request = {
          letters: letters,
          min: parseInt(min),
        };
        fetch("/api/letters-to-words", {
          method: "POST",
          body: JSON.stringify(request),
        })
          .then((res) => res.json())
          .then(
            (res) => setWords(res),
            (error) => console.log(error),
          );
      }
    };
    document.addEventListener("keydown", listener);
    return () => {
      document.removeEventListener("keydown", listener);
    };
  }, [min, words, letters]);

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
              /api/letters-to-words
            </Typography>
            <Typography variant="h5" component="div">
              Letters to Words
            </Typography>
            <Typography variant="body2">
              Given a set of letters, return a list of words that can be made
              from those letters.
            </Typography>
          </CardContent>
        </Card>
      </Box>
      <Box display="flex">
        <Box
          m="auto"
          component="form"
          sx={{
            "& .MuiTextField-root": { m: 1, width: "25ch" },
          }}
          noValidate
          autoComplete="off"
        >
          <TextField
            value={letters}
            onChange={(e) => setLetters(e.target.value)}
            label="Words"
            variant="outlined"
            inputProps={{
              "aria-label": "letters from which to generate words",
            }}
          />
          <TextField
            select
            value={min}
            onChange={(e) => setMin(e.target.value)}
            label="Minimum Word Size"
            variant="outlined"
            inputProps={{
              "aria-label": "minimus size of each word",
            }}
          >
            <MenuItem key="3" value="3">
              3
            </MenuItem>
            <MenuItem key="4" value="4">
              4
            </MenuItem>
          </TextField>
        </Box>
      </Box>
      <Box m="auto" sx={{ minWidth: 275, maxWidth: 500 }}>
        <Masonry columns={3} spacing={1}>
          {words.map((word) => (
            <Item key={word}>
			  {word}
            </Item>
          ))}
        </Masonry>
      </Box>
    </React.Fragment>
  );
};

export default LettersToWords;
