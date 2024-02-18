use std::fs;
use std::path::PathBuf;
use std::io;

use regex;
use rayon::prelude::*;


#[derive(Debug)]
pub struct SearchState {
	request : String,
	results : Vec<String>,
	files : Vec<PathBuf>,
	origin : PathBuf,
}


impl SearchState {
	pub fn new(origin : &str) -> Self {
		return SearchState {
			request : String::new(),
			results : Vec::new(),
			files : Vec::new(),
			origin : PathBuf::from(origin),
		}
	}


	pub fn get_file_count(&self) -> usize {
		return self.files.len();
	}


	pub fn update_request(&mut self, new_request : String) {
		self.request = new_request.clone();
		self.results = Vec::new();
		let _ = self.search();
	}


	pub fn get_results(&self) -> Vec<String> {
		return self.results.clone();
	}


	fn get_files(&mut self) -> Result<(), io::Error> {
		//let mut search_loactions : Vec<PathBuf> = vec![self.origin.clone()];
		self.files = get_tree(self.origin.clone(), 10)?;
		/*
		let mut i = 0;
		let mut count = search_loactions.len();

		while i < count {

			let ls = fs::read_dir(&search_loactions[i])?;
			for file in ls {
				let file = file?;

				if file.path().is_dir() {
					search_loactions.push( file.path().clone() );
					count += 1;
				} else {
					self.files.push( file.path() );
				}
			}
			i += 1;
		}
		*/
		Ok(())
	}


	fn search(&mut self) -> Result<(), io::Error> {
		if self.files.len() == 0 {
			let _ = self.get_files();
		}

		self.results = Vec::new();

		let search_exp = regex_from_wildcards(&self.request);

		self.results = self.files.par_iter()
			.filter(|file| {
				let name_string = pathbuf_to_string(&file);
				search_exp.is_match(&name_string)
			})
			.map(|file| pathbuf_to_string(&file)).collect();

		Ok(())
	}
}


fn get_tree(path : PathBuf, depth : usize) -> Result<Vec<PathBuf>, io::Error> {
	if depth == 0 {return Ok(Vec::new());}

	let mut res : Vec<PathBuf> = Vec::new();
	for file in fs::read_dir(path)? {
		let file_path = file?.path();
		if file_path.is_dir() {
			let mut sub_dirs = match get_tree(file_path, depth - 1){
				Ok(result) => result,
				Err(_) => Vec::new(),
			};
			res.append(&mut sub_dirs);
		} else {
			res.push(file_path);
		}
	}
	return Ok(res);
}


fn pathbuf_to_string(pb : &PathBuf) -> String {
	let os_string = pb.clone().into_os_string();
	return os_string.into_string().unwrap();
}


fn regex_from_wildcards(string : &str) -> regex::Regex {

	let mut regexed_string = str::replace(string, ".", "\\.");
	regexed_string = str::replace(&regexed_string, "*", ".*");
	regexed_string = str::replace(&regexed_string, "?", ".");

	return regex::Regex::new(&regexed_string).unwrap();
}