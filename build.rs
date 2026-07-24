use std::{
    env, fs, io,
    path::{Path, PathBuf},
};

const TEST_DIR: &str = "tests/chapter_tests";
const MAX_CHAPTER: u8 = 8;
fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = PathBuf::from(out_dir).join("generated_tests.rs");
    let mut out = String::new();

    println!("cargo:rerun-if-changed=tests/chapter_tests");

    visit_test_files(Path::new(TEST_DIR), &mut out).unwrap();
    fs::write(&dest_path, out).unwrap();
}

fn visit_test_files(path: &Path, out: &mut String) -> io::Result<()> {
    if path.is_dir() {
        let entries = fs::read_dir(path)?;
        for entry in entries {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_test_files(&path, out)?;
            } else {
                if path
                    .extension()
                    .is_some_and(|s| s.eq_ignore_ascii_case("c"))
                {
                    generate_test(&path, out)?;
                }
            }
        }
    }
    Ok(())
}

fn generate_test(path: &Path, out: &mut String) -> io::Result<()> {
    let name = path
        .with_extension("")
        .to_str()
        .unwrap()
        .replace("/", "_")
        .replace("-", "_")
        .replace(".", "_");
    let path_str = path.display().to_string().replace("\\", "/");
    let ignore = path.iter().nth(2).is_some_and(|n| {
        n.to_str()
            .and_then(|s| s.strip_prefix("chapter_"))
            .and_then(|s| s.parse::<u8>().ok())
            .is_some_and(|ch| ch > MAX_CHAPTER)
    });
    let ignore_str = if ignore { "\n#[ignore]" } else { "" };
    let fun = &format!(
        r#"
#[test]{}
fn {}() {{ run_single_test("{}"); }}"#,
        ignore_str, name, path_str
    );
    out.push_str(fun);
    Ok(())
}
