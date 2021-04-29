mod fsutils {
    pub struct FileContents(pub String);
    pub struct FilePath<'a>(pub &'a str);

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

    impl Error {
        pub fn msg(&self) -> String {
            match *self {
                Self::FileReadError(ref io_err) => io_err.to_string(),
            }
        }
    }

    impl From<std::io::Error> for Error {
        fn from(io_err: std::io::Error) -> Self { Self::FileReadError(io_err) }
    }
}

mod driver {
    pub fn run(_: crate::fsutils::FileContents) -> Result<(), crate::error::Error> {
        todo!();
    }

    pub fn read_and_run(path: crate::fsutils::FilePath) -> Result<(), crate::error::Error> {
        let contents = crate::fsutils::read(path)?;
        run(contents)
    }
}

fn main() {
    for arg in std::env::args().skip(1) {
        match driver::read_and_run(fsutils::FilePath(&arg)) {
            Ok(()) => (),
            Err(error) => {
                eprintln!("error running file '{}': {}", arg, error.msg());
            }
        }
    }
}
