mod fsutils {
    struct FileContents(String);
    struct FilePath(String);

    fn read(path: FilePath) -> Result<FileContents, FileReadError> {
        todo!();
    }
}
mod driver {
    fn run(contents: fsutils::FileContents) -> bool {
        todo!();
    }

    fn read_and_run(path: fsutils::FilePath) -> Result<(), Error> {
        todo!();
    }
}

fn main() {
    for arg in std::env::args() {
        driver::read_and_run(fsutils::FilePath(arg));
    }
}
