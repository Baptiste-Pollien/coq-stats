/// Function to get information from the system


pub mod system {
    use std::fs::metadata;
    use std::path::Path;
    use std::ffi::OsStr;

    /// Test if the string corresponds to a folder
    pub fn is_folder(path: &String) -> bool {
        let md = metadata(path).unwrap();
        md.is_dir()
    }

    /// Test if the string corresponds to a Coq file
    pub fn is_coq_file(path: &String) -> bool {
        let ext = Path::new(&path)
                    .extension()
                    .and_then(OsStr::to_str);

        ext == Some("v")
    }
}
