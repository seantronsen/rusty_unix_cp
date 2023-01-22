use std::fs;
use std::io::{self, Read, Write};
use std::path::PathBuf;
use std::vec::IntoIter;

pub struct Config {
    pub sources: Option<Vec<io::Result<PathBuf>>>,
    pub target: Option<PathBuf>,
    pub target_isdir: bool,
}

impl Config {
    pub fn build(args: Vec<String>) -> Result<Self, String> {
        let total_args = args.len();
        if total_args < 2 {
            return Err(String::from("missing required argument `source`"));
        }

        let mut sources: Vec<io::Result<PathBuf>> = vec![];
        let mut iter = args.iter();
        iter.next();

        for _ in 1..total_args-1 {
            let arg = match iter.next() {
                Some(str) => PathBuf::from(str),
                None => return Err(String::from("`source` received arg with value `None`")),
            };
            sources.push(Ok(arg));
        }
        let target = match iter.next() {
            Some(val) => PathBuf::from(val),
            None => return Err(String::from("`target` received arg with value `None`")),
        };

        let target_isdir = target.is_dir();

        Ok(Self {
            sources: Some(sources),
            target: Some(target),
            target_isdir,
        })
    }
}

pub fn recursive_copy(
    //mut sources: impl Iterator<Item = io::Result<PathBuf>>,
    mut sources: IntoIter<io::Result<PathBuf>>,
    target_dir: PathBuf,
) -> io::Result<()> {
    while let Some(source) = sources.next() {
        let source = match source {
            Ok(source) => source,
            Err(e) => return Err(e),
        };

        let source_name = source
            .file_name()
            .expect("panic caused by issue with OsString");
        let mut next_target = target_dir.clone();
        next_target.push(source_name);

        if source.is_dir() {
            let dir_contents: Vec<io::Result<PathBuf>> = source
                .read_dir()?
                .map(|entry| io::Result::Ok(entry?.path()))
                .collect();
            recursive_copy(dir_contents.into_iter(), next_target)?;
        } else if source.is_file() {
            if !target_dir.try_exists()? {
                let mut builder = fs::DirBuilder::new();
                builder.recursive(true);
                builder.create(&target_dir)?;
            }
            copy(source, next_target)?;
        } else {
            panic!("source was not a file or directory! this program doesn't deal with symlinks");
        }
    }

    Ok(())
}

pub fn copy(source_file: PathBuf, target_file: PathBuf) -> io::Result<()> {
    let mut contents: Vec<u8> = vec![];
    fs::File::open(source_file)?.read_to_end(&mut contents)?;
    fs::File::create(target_file)?.write(&contents)?;
    Ok(())
}
