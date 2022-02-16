use futures::lock::Mutex;
use std::collections::HashSet;
use std::env;

use serde::{Deserialize, Serialize};

use marshians_fn::{sudoku::solve, words::dictionary, words::words};

#[macro_use]
extern crate rocket;
use rocket::{fs::FileServer, http::Status, response::status, serde::json::Json};

pub struct State {
	pub dictionary: HashSet<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SudokuSolution {
	pub original: String,
	pub solution: String,
}

#[post("/sudoku-solver", data = "<board>")]
async fn sudoku_solver(board: &str) -> Result<Json<SudokuSolution>, status::Custom<String>> {
	match solve(board) {
		Ok(solution) => Ok(Json(SudokuSolution {
			original: board.to_string(),
			solution: solution,
		})),
		Err(err) => Err(status::Custom(Status::BadRequest, err.to_string())),
	}
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LetterToWordsBody<'r> {
	pub letters: &'r str,
	pub min: usize,
}

#[post("/letters-to-words", data = "<body>")]
async fn letters_to_words(
	body: Json<LetterToWordsBody<'_>>,
	state: &rocket::State<Mutex<State>>,
) -> Result<Json<Vec<String>>, status::Custom<String>> {
	let state = state.lock().await;
	Ok(Json(words(&state.dictionary, body.letters, body.min)))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	match env::var("PORT") {
		Ok(port) => env::set_var("ROCKET_PORT", port),
		Err(_) => (),
	};
	let state = State {
		dictionary: dictionary()?,
	};
	rocket::build()
		.manage(Mutex::new(state))
		.mount("/", FileServer::from("./ui"))
		.mount("/api", routes![sudoku_solver, letters_to_words])
		.launch()
		.await?;
	Ok(())
}
