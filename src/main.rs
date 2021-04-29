mod fsutils {
    pub struct FileContents(pub String);
    pub struct FilePath(pub String);

    pub fn read(path: FilePath) -> Result<FileContents, std::io::Error> {
        match std::fs::read_to_string(path.0) {
            Ok(o) => Ok(FileContents(o)),
            Err(e) => Err(e),
        }
    }
}

mod error {
    pub enum Error {
        FileReadError(std::io::Error),
    }

    impl From<std::io::Error> for Error {
        fn from(io_err: std::io::Error) -> Self {
            Self::FileReadError(io_err)
        }
    }
}

mod driver {
    pub fn run(contents: crate::fsutils::FileContents) -> Result<(), crate::error::Error> {
        todo!();
    }

    pub fn read_and_run(path: crate::fsutils::FilePath) -> Result<(), crate::error::Error> {
        let contents = crate::fsutils::read(path)?;
        run(contents)
    }
}

fn main() {
    for arg in std::env::args() {
        driver::read_and_run(fsutils::FilePath(arg));
    }
}
