#[cfg(test)]
mod tests {
    use std::{fs::{self, File}, path::Path, process::{self, Command}};
    use regex::Regex;

    struct TestConfig {
        valid: bool,
        return_val: Option<i32>,
        include: Vec<String>
    }

    struct Cleanup {
        tmp_out: String
    }

    impl Drop for Cleanup {
        fn drop(&mut self) {
            let force_clean = true;
            if force_clean {
                let _ = fs::remove_dir_all(&self.tmp_out);
            }
            let tmp_out: String = format!("./.tmp.build.{}.out", process::id());
            // Clear the test output directory if it is empty
            if fs::read_dir(&tmp_out).is_ok_and(|d| d.count() == 0) {
                fs::remove_dir_all(&tmp_out).unwrap();
            }
        }
    }

    fn get_config(test_file_path: &str) -> Option<TestConfig> {
        let content = fs::read_to_string(test_file_path).unwrap();

        let is_invalid = content.contains("// test-directive invalid");
        let is_valid = content.contains("// test-directive valid");

        if !is_invalid && !is_valid {
            return None
        }

        let re = Regex::new(r"//\s*test-directive\s+return_code:\s*(\d+)").ok()?;

        let return_val = re.captures(&content)
            .and_then(|cap| cap.get(1))
            .and_then(|m| m.as_str().parse::<i32>().ok());

        return Some(TestConfig { valid: is_valid, return_val, include: Vec::new() })
    }

    fn run_single_test(test_file_path: &str) {
        let config = get_config(test_file_path);
        if config.is_none() {
            return;
        }
        let config = config.unwrap();

        let compiler = env!("CARGO_BIN_EXE_compiler");

        let tmp_out = format!(
            "./.tmp.build.{}.out/{}/",
            std::process::id(),
            test_file_path.replace("/", "_").trim_end_matches(".c")
        );

        let _cleanup = Cleanup {
            tmp_out: tmp_out.clone()
        };

        // Preprocessing
        let _ = std::fs::create_dir_all(&tmp_out);
        let preprocessed_file_path = Path::new(test_file_path).with_extension("i");
        let preprocessed_file_path = tmp_out.clone()
            + preprocessed_file_path
                .file_name()
                .unwrap()
                .to_str()
                .unwrap();
        let mut pp_cmd = Command::new("gcc");
        let pp_status = pp_cmd
            .arg("-E")
            .arg("-P")
            .arg(test_file_path)
            .arg("-o")
            .arg(&preprocessed_file_path)
            .status();

        assert_eq!(pp_status.unwrap().code().unwrap(), 0);

        let log_file = File::create(tmp_out.clone() + "debug.log").unwrap();
        // Compiling
        let mut compile_cmd = Command::new(compiler);
        let compiler_status = compile_cmd
            .arg(&preprocessed_file_path)
            .arg("-l")
            .arg("--asm")
            .arg("--ast")
            .arg("--debug-level=Trace")
            .stderr(log_file)
            .status()
            .unwrap();

        assert_eq!(compiler_status.success(), config.valid);

        if !config.valid {
            return
        }

        // gcc "$ASSEMBLY_FILE" -o "$EXECUTABLE_NAME"
        // Compile the executable
        let cc_cmd = Command::new("gcc")
            .arg(Path::new(&preprocessed_file_path).with_extension("s"))
            .arg("-o")
            .arg(Path::new(&preprocessed_file_path).with_extension(""))
            .status()
            .unwrap();

        assert_eq!(cc_cmd.success(), config.valid);
        
        let exe_cmd = Command::new(Path::new(&preprocessed_file_path).with_extension(""))
            .status().unwrap();

        if let Some(return_val) = &config.return_val {
            assert_eq!(exe_cmd.code().unwrap(), *return_val);
        } 

        fs::remove_dir_all(&tmp_out).unwrap();
    }

    include!(concat!(env!("OUT_DIR"), "/generated_tests.rs"));
}
