use cursive;
use cursive::CursiveExt;
use cursive::views::{SelectView, LinearLayout, Dialog, EditView};
use cursive::traits::{Nameable, Resizable};

fn main() {
	let _ = build_interface();
}


#[derive(Debug)]
struct SearchState {
	request : String,
	results : Vec<String>,
}


impl SearchState {
	pub fn new() -> Self {
		return SearchState {
			request : String::new(),
			results : Vec::new(),
		}
	}

	pub fn update_request(&mut self, new_request : String) {
		self.request = new_request.clone();
		self.results = vec!{String::from("dummy"), String::from("test")};
		self.results.push(new_request);
	}

	pub fn get_results(&self) -> Vec<String> {
		return self.results.clone();
	}

}


fn update_search(s : &mut cursive::Cursive, search : &str, _len : usize) {
	let search_state : &mut SearchState = s.user_data().unwrap();
	//run search on the current request (maybe think of a smart way to only look for the diffs)

	search_state.update_request(String::from(search));

	//display the results
	let results = search_state.get_results();

	//TODO: make this more elegant and maybe more performant
	s.call_on_name("results", |view : &mut SelectView<String>| {
		view.clear();
		for item in results.iter() {
			view.add_item(item.clone(), item.clone());
		}
	});

}


fn build_interface() -> cursive::Cursive{
	let mut interface = cursive::Cursive::default();

	interface.set_user_data(SearchState::new());

	//quit callback
	interface.add_global_callback(cursive::event::Key::Esc, |s| s.quit());

	let input = Dialog::around(
			EditView::new()
			.on_edit(update_search)
			.with_name("search string")
		)
		.title("search string")
		.fixed_width(80);

	let select = SelectView::<String>::new()
		.with_name("results");

	let app_layout = LinearLayout::vertical()
		.child(input)
		.child(select);


	interface.add_layer(app_layout);

	interface.run();
	return interface;
}