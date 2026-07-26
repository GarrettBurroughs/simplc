#[cfg(test)]
mod tests {
    use std::{
        collections::HashSet,
        fs::{self, File},
        path::{Path, PathBuf},
        process::{self, Command},
    };
    use regex::Regex;

    struct TestConfig {
        valid: bool,
        return_val: Option<i32>,
        include: Vec<String>,
    }

    struct Cleanup {
        tmp_out: String,
    }

    impl Drop for Cleanup {
        fn drop(&mut self) {
            let force_clean = false;
            if force_clean {
                let _ = fs::remove_dir_all(&self.tmp_out);
            }
            let tmp_out: String = format!("./.tests/.tmp.build.{}.out", process::id());
            // Clear the test output directory if it is empty
            if fs::read_dir(&tmp_out).is_ok_and(|d| d.count() == 0) {
                fs::remove_dir_all(&tmp_out).unwrap();
            }
        }
    }

    fn get_config(test_file_path: &str) -> Option<TestConfig> {
        let content = fs::read_to_string(test_file_path).ok()?;

        let is_invalid = content.contains("// test-directive invalid");
        let is_valid = content.contains("// test-directive valid");

        if !is_invalid && !is_valid {
            return None;
        }

        let re_val = Regex::new(r"//\s*test-directive\s+return_code:\s*(-?\d+)").ok()?;
        let return_val = re_val
            .captures(&content)
            .and_then(|cap| cap.get(1))
            .and_then(|m| m.as_str().parse::<i32>().ok());

        let re_inc = Regex::new(r"//\s*test-directive\s+include\s+(\S+)").ok()?;
        let parent_dir = Path::new(test_file_path).parent()?;

        let mut include = Vec::new();
        for cap in re_inc.captures_iter(&content) {
            if let Some(inc_match) = cap.get(1) {
                let inc_str = inc_match.as_str();

                #[cfg(target_os = "linux")]
                if inc_str.contains("_osx") || inc_str.contains("_mac") {
                    continue;
                }

                #[cfg(target_os = "macos")]
                if inc_str.contains("_linux") {
                    continue;
                }

                let inc_path = parent_dir.join(inc_str);
                if let Some(path_str) = inc_path.to_str() {
                    include.push(path_str.to_string());
                }
            }
        }

        Some(TestConfig {
            valid: is_valid,
            return_val,
            include,
        })
    }

    fn run_single_test(test_file_path: &str) {
        let config = match get_config(test_file_path) {
            Some(c) => c,
            None => return,
        };

        let compiler = env!("CARGO_BIN_EXE_compiler");

        let tmp_out = format!(
            "./.tests/.tmp.build.{}.out/{}/",
            process::id(),
            test_file_path.replace("/", "_").replace("\\", "_").trim_end_matches(".c")
        );

        let _cleanup = Cleanup {
            tmp_out: tmp_out.clone(),
        };

        let _ = fs::create_dir_all(&tmp_out);

        let main_path_buf = PathBuf::from(test_file_path);
        let main_stem = main_path_buf.file_stem().unwrap().to_str().unwrap();
        let main_preprocessed = format!("{}{}.i", tmp_out, main_stem);

        // Preprocessing main file
        let pp_status = Command::new("gcc")
            .arg("-E")
            .arg("-P")
            .arg(test_file_path)
            .arg("-o")
            .arg(&main_preprocessed)
            .status();

        assert_eq!(pp_status.unwrap().code().unwrap(), 0);

        let log_file = File::create(format!("{}debug.log", tmp_out)).unwrap();

        // Compiling main file with custom compiler
        let mut compile_cmd = Command::new(compiler);
        let compiler_status = compile_cmd
            .arg(&main_preprocessed)
            .arg("-l")
            .arg("--asm")
            .arg("--ast")
            .arg("--debug-level=Trace")
            .stderr(log_file)
            .status()
            .unwrap();

        assert_eq!(compiler_status.success(), config.valid);

        if !config.valid {
            let _ = fs::remove_dir_all(&tmp_out);
            return;
        }

        let main_s = format!("{}{}.s", tmp_out, main_stem);

        let mut gcc_link_args: Vec<PathBuf> = vec![PathBuf::from(&main_s)];
        let mut processed_files = HashSet::new();
        processed_files.insert(main_path_buf.clone());

        let mut includes_to_process = config.include.clone();

        while let Some(inc_str) = includes_to_process.pop() {
            let inc_path = PathBuf::from(&inc_str);
            if processed_files.contains(&inc_path) {
                continue;
            }
            processed_files.insert(inc_path.clone());

            let ext = inc_path.extension().and_then(|s| s.to_str()).unwrap_or("");

            if ext == "s" {
                gcc_link_args.push(inc_path);
            } else if ext == "c" {
                let path_str = inc_path.to_str().unwrap_or("");
                if path_str.contains("helper_libs") {
                    gcc_link_args.push(inc_path);
                } else {
                    let inc_stem = inc_path.file_stem().unwrap().to_str().unwrap();
                    let inc_preprocessed = format!("{}{}.i", tmp_out, inc_stem);

                    let inc_pp_status = Command::new("gcc")
                        .arg("-E")
                        .arg("-P")
                        .arg(&inc_path)
                        .arg("-o")
                        .arg(&inc_preprocessed)
                        .status()
                        .unwrap();

                    assert_eq!(inc_pp_status.code().unwrap(), 0);

                    let inc_log_file = File::create(format!("{}debug_{}.log", tmp_out, inc_stem)).unwrap();
                    let inc_compiler_status = Command::new(compiler)
                        .arg(&inc_preprocessed)
                        .arg("-l")
                        .arg("--asm")
                        .arg("--ast")
                        .arg("--debug-level=Trace")
                        .stderr(inc_log_file)
                        .status()
                        .unwrap();

                    assert!(
                        inc_compiler_status.success(),
                        "Failed to compile included C file with compiler: {}",
                        inc_str
                    );

                    let inc_s = format!("{}{}.s", tmp_out, inc_stem);
                    gcc_link_args.push(PathBuf::from(&inc_s));

                    if let Some(inc_config) = get_config(&inc_str) {
                        for sub_inc in inc_config.include {
                            includes_to_process.push(sub_inc);
                        }
                    }
                }
            }
        }

        // Link executable with GCC
        let exe_path = format!("{}exe", tmp_out);
        let mut cc_cmd = Command::new("gcc");
        for arg in &gcc_link_args {
            cc_cmd.arg(arg);
        }
        cc_cmd.arg("-o").arg(&exe_path);

        let cc_status = cc_cmd.status().unwrap();
        assert!(cc_status.success(), "GCC linking failed");

        let exe_cmd = Command::new(&exe_path).status().unwrap();

        if let Some(return_val) = &config.return_val {
            assert_eq!(exe_cmd.code().unwrap(), *return_val);
        }

        let _ = fs::remove_dir_all(&tmp_out);
    }

    include!(concat!(env!("OUT_DIR"), "/generated_tests.rs"));
}
