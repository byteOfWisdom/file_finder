use std::fs;
use std::path::PathBuf;
use std::io;

//use serde::{Deserialize, Serialize};
use regex;

#[derive(Debug)]
pub struct SearchState {
	request : String,
	results : Vec<String>,
	files : Vec<PathBuf>,
	config : SearchConfig,
}


impl SearchState {
	pub fn new() -> Self {
		return SearchState {
			request : String::new(),
			results : Vec::new(),
			files : Vec::new(),
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

	fn get_files(&mut self, root : &str, depth : usize) -> Result<(), io::Error> {
		let mut search_loactions : Vec<PathBuf> = vec![PathBuf::from(root)];

		//should I even clear the file buffer?
		self.files = Vec::new();

		let mut i = 0;

		while i < search_loactions.len() {

			let ls = fs::read_dir(&search_loactions[i])?;
			for file in ls {
				let file = file?;

				if file.path().is_dir() {
					search_loactions.push(file.path());
				} else {
					self.files.push(file.path());
				}
			}
			i += 1;
		}

		Ok(())
	}


	fn search(&mut self) -> Result<(), io::Error> {
		if self.files.len() == 0 {
			let _ = self.get_files(".", 0);
		}

		self.results = Vec::new();

		self.files.iter()
			.filter(|file| is_match(&self.request, &file))
			.for_each(|file| self.results.push(file.clone().into_os_string().into_string().unwrap()));

		Ok(())
	}
}


fn is_match(filter : &str, name : &PathBuf) -> bool {
	let name_string = &name.clone().into_os_string().into_string().unwrap();
	return name_string.contains(filter);
}



#[derive(Debug)]
struct SearchConfig {
	pub search_roots : Vec<String>,
	pub max_depth : usize,
}


impl SearchConfig {
	pub fn default() -> Self {
		SearchConfig {
			search_roots : vec!["~/".to_string(), "./".to_string()],
			max_depth : 5,
		}
	}
}