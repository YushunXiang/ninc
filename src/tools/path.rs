pub fn get_save_path(name: &str) -> String {
  let self_file = std::env::current_exe().unwrap();
  let self_path = self_file.parent().unwrap();
  String::from(self_path.to_str().unwrap()) + "/" + name
}
