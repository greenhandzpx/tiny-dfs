use once_cell::sync::Lazy;

/// Note that local dir must NOT have '/' at the end
static mut LOCAL_DIR: Lazy<String> = Lazy::new(|| String::new());

// fn local_dir() -> &'static str {
//     unsafe { &LOCAL_DIR }
// }

pub fn set_local_dir(dir: String) {
    unsafe { *LOCAL_DIR = dir }
}

pub fn global_to_local(global_path: &str) -> String {
    unsafe { LOCAL_DIR.to_owned() + global_path }
    // format!("{}{}", unsafe { LOCAL_DIR }, global_path)
}

pub fn local_to_global<'a>(local_path: &'a str) -> &'a str {
    &local_path[unsafe { LOCAL_DIR.len() }..]
}
