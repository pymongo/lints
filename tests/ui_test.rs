const UI_TEST_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/ui_test");

fn ui_test_single_file(path: &str) {
    let path = std::path::Path::new(path);
    let mut rs_file_path = if path.is_absolute() {
        assert!(path.starts_with(UI_TEST_DIR));
        path.to_path_buf()
    } else {
        // canonicalize is similar to realpath command
        std::path::Path::new(UI_TEST_DIR)
            .canonicalize()
            .unwrap()
            .join(path)
    };
    if rs_file_path.extension().is_none() {
        rs_file_path.set_extension("rs");
    }
    dbg!(&rs_file_path);
    let mut stderr_file_path = rs_file_path.clone();
    assert!(stderr_file_path.set_extension("stderr"));

    let expected_stderr = std::fs::read_to_string(stderr_file_path).unwrap();
    let stderr = std::process::Command::new("cargo")
        .arg("run")
        // ignore stdout
        .arg("--quiet")
        .arg("--")
        // cargo check only
        .arg("--emit=metadata")
        .arg("--crate-type=lib")
        .arg("--allow=dead_code")
        .arg("--allow=unused_variables")
        .arg("-L")
        .arg(env!("STD_DYLIB_PATH"))
        .arg(rs_file_path)
        .output()
        .unwrap()
        .stderr;
    let stderr = unsafe { String::from_utf8_unchecked(stderr) };
    println!("{}", stderr);
    // assert_eq!(stderr, expected_stderr);
}

/// similar to linux [ftw(3)](https://man7.org/linux/man-pages/man3/ftw.3.html)
fn walk_dir_iterative<F: Fn(&std::path::PathBuf)>(dir_abs_path: String, file_path_handler: F) {
    let mut queue = std::collections::VecDeque::new();
    queue.push_back(dir_abs_path);
    while let Some(dir_abs_path) = queue.pop_front() {
        for dirent in std::fs::read_dir(dir_abs_path).unwrap().flatten() {
            let path = dirent.path();
            if path.is_dir() {
                queue.push_back(path.to_str().unwrap().to_string());
            } else {
                file_path_handler(&path);
            }
        }
    }
}

#[test]
#[ignore]
fn test_ui_test_single_file() {
    ui_test_single_file("fn_name_is_foo");
    // ui_test_single_file("check_enum_size.rs");
}

#[test]
fn run_all_ui_test() {
    walk_dir_iterative(UI_TEST_DIR.to_string(), |path| {
        if let Some(ext) = path.extension() {
            if ext == "rs" {
                ui_test_single_file(path.to_str().unwrap());
            }
        }
    });
}
