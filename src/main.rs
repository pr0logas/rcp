use std::fs;
use std::env;
use serde_json;
use regex::Regex;
use std::path::Path;

fn determinate_file_size(file: &str) -> u64 {
	fs::metadata(file).unwrap().len()
}

//fn check_if_path_file_exits(path: &str) {
//	Path::new(path).exists()
//}

fn determinate_is_it_file_or_directory(arg: &str) -> &str {
	let file = "File";
	let dir = "Directory";
	let re = Regex::new(r"/").unwrap();
	if re.is_match(arg) {
		return dir;
	}
	return file;
}

fn collect_user_arguments() -> Vec<String> {
	env::args().collect()
}

fn check_if_arguments_count_valid(args: &Vec<String>) -> bool {
	if args.len() == 3 {
		return true
	}
	help();
	return false
}

//fn get_file_metadata(file: &str) -> {
//	let attr = fs::metadata("Cargo.toml")?;
//	Ok(())
//}

fn get_current_working_dir() -> String {
    let res = env::current_dir();
    match res {
    	Ok(path) => path.into_os_string().into_string().unwrap(),
    	Err(_) => "FAILED".to_string()
    }
}

fn help() {
	println!("Examples:");
    println!("rcp [srcfile] [destfile]");
    println!("rcp [srcdir]/[srcfile] [destdir]/[destfile]");
}


fn main() {
	let working_dir = get_current_working_dir();
	println!("{:#?}", working_dir);
    let args: Vec<String> = collect_user_arguments();

    if check_if_arguments_count_valid(&args) {
    	let arg1 = &args[1];
    	let arg2 = &args[2];
    	println!("{:#?}", determinate_is_it_file_or_directory(&arg1));
    }
}