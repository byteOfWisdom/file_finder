use cursive;
use cursive::CursiveExt;
use cursive::views::{SelectView, LinearLayout, Dialog, EditView, TextView, ViewRef, ScrollView};
use cursive::traits::{Nameable, Resizable};

use cli_clipboard;

mod search;
use search::SearchState;

use std::env;

fn main() {
	let mut base_path = String::from(".");

	let argv = env::args().collect::<Vec<String>>();
	if argv.len() > 1 {
		base_path = argv[1].clone();
	}

	run_interface(&base_path);
}


fn update_search(s : &mut cursive::Cursive, search : &str) {
	let search_state : &mut SearchState = s.user_data().unwrap();
	//run search on the current request (maybe think of a smart way to only look for the diffs)

	search_state.update_request(String::from(search));

	//display the results
	let results = search_state.get_results();
	let fc = search_state.get_file_count();

	//TODO: make this more elegant and maybe more performant
	s.call_on_name("results", |view : &mut SelectView<String>| {
		view.clear();
		for item in results.iter() {
			view.add_item(item.clone(), item.clone());
		}
	});

	let mc = results.len();
	//update the debug info
	let text = format!("found {} files, {} of which match the search", fc, mc);
	let mut text_view : ViewRef<TextView> = s.find_name("info").unwrap();
	text_view.set_content(text);
}


fn run_interface(origin : &str) {
	let mut interface = cursive::Cursive::default();

	interface.set_user_data(SearchState::new(origin));

	//quit callback
	interface.add_global_callback(cursive::event::Key::Esc, |s| s.quit());

	let input = Dialog::around(
			EditView::new()
			.on_submit(update_search)
			.with_name("search string")
		)
		.title("search string")
		.fixed_width(80)
		.fixed_height(3);

	let select = SelectView::<String>::new()
		.on_submit(handle_selection)
		.with_name("results");

	let info = TextView::new("info")
		.with_name("info");


	let app_layout = LinearLayout::vertical()
		.child(input)
		.child(info)
		.child(ScrollView::new(select));


	interface.add_layer(app_layout);

	interface.run();
}

fn handle_selection(s : &mut cursive::Cursive, entry : &str) {
	//for now, just copy the string to clipboard and quit.
	//TODO: change this behaviour to multiple options on what to do
	cli_clipboard::set_contents(String::from(entry).to_owned()).unwrap();
	s.quit();
}