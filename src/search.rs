use std::fs;
use std::io;

use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct SearchState {
	request : String,
	results : Vec<String>,
	config : SearchConfig,
}


impl SearchState {
	pub fn new() -> Self {
		return SearchState {
			request : String::new(),
			results : Vec::new(),
			config : SearchConfig::default(),
		}
	}

	pub fn update_request(&mut self, new_request : String) {
		self.request = new_request.clone();
		self.results = vec!{String::from("dummy"), String::from("test")};
		let _ = self.search();
	}

	pub fn get_results(&self) -> Vec<String> {
		return self.results.clone();
	}


	fn search(&mut self) -> Result<(), io::Error>{
		//let search_loactions : Vec<fs::Path> = Vec::new();
		


		self.results = Vec::new();
		let ls = fs::read_dir(&self.request)?;
		for file in ls {
			let file = file?;

			if file.path().is_dir() {
				//add subdirectories to the todo and read those later
			}
			
			self.results.push(file.file_name().into_string().unwrap());
		}

		Ok(())
	}


}


#[derive(Debug)]
struct SearchConfig {
	pub search_roots : Vec<String>,
}


impl SearchConfig {
	pub fn default() -> Self {
		SearchConfig {
			search_roots : vec!["~/".to_string(), "./".to_string()],
		}
	}
}