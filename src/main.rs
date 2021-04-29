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
        ParseError(crate::ast::ParseError),
        RuntimeError(crate::interpret::RuntimeError),
    }

    impl Error {
        pub fn msg(&self) -> String {
            match *self {
                Self::FileReadError(ref io_err) => io_err.to_string(),
                Self::ParseError(_) => "parsing error".to_string(),
                Self::RuntimeError(_) => "runtime error".to_string(),
            }
        }
    }

    impl From<std::io::Error> for Error {
        fn from(e: std::io::Error) -> Self { Self::FileReadError(e) }
    }
    impl From<crate::ast::ParseError> for Error {
        fn from(e: crate::ast::ParseError) -> Self { Self::ParseError(e) }
    }
    impl From<crate::interpret::RuntimeError> for Error {
        fn from(e: crate::interpret::RuntimeError) -> Self { Self::RuntimeError(e) }
    }
}

mod driver {
    pub fn run(contents: crate::fsutils::FileContents) -> Result<(), crate::error::Error> {
        let ast = crate::ast::AST::new(contents)?;
        crate::interpret::interpret(ast)?;

        Ok(())
    }

    pub fn read_and_run(path: crate::fsutils::FilePath) -> Result<(), crate::error::Error> {
        let contents = crate::fsutils::read(path)?;
        run(contents)
    }
}

mod ast {
    pub struct AST;
    pub struct ParseError;

    impl AST {
        pub fn new(tokens: crate::fsutils::FileContents) -> Result<AST, ParseError> {
            todo!();
        }
    }
}

mod interpret {
    pub struct RuntimeError;

    pub fn interpret(contents: crate::ast::AST) -> Result<(), RuntimeError> {
        todo!();
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
