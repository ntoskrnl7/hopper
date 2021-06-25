use deque;
use sender;
use std::path::Path;
use std::{cmp, fs, io};

#[derive(Debug)]
pub enum Placement<T> {
    Memory(T),
    Disk(usize),
}

impl<T> Placement<T> {
    pub fn extract(self) -> Option<T> {
        match self {
            Placement::Memory(elem) => Some(elem),
            Placement::Disk(_) => None,
        }
    }
}

pub type Queue<T> = deque::Queue<Placement<T>, sender::SenderSync>;

pub fn read_seq_num(data_dir: &Path) -> io::Result<usize> {
    let mut max = 0;
    for directory_entry in fs::read_dir(data_dir)? {
        let num = directory_entry?
            .file_name()
            .to_str()
            .unwrap()
            .parse::<usize>()
            .unwrap();
        max = cmp::max(num, max);
    }
    Ok(max)
}

pub fn read_seq_num_min(data_dir: &Path) -> io::Result<usize> {
    let mut min = usize::max_value();
    let mut worked = false;
    for directory_entry in fs::read_dir(data_dir)? {
        let num = directory_entry?
            .file_name()
            .to_str()
            .unwrap()
            .parse::<usize>()
            .unwrap();
        worked = true;
        min = cmp::min(num, min);
    }
    assert!(worked);
    Ok(min)
}

pub fn clear_directory(data_dir: &Path) -> io::Result<()> {
    if data_dir.is_dir() {
        for directory_entry in fs::read_dir(data_dir)? {
            let de = directory_entry?;
            if let Ok(meta) = fs::metadata(de.path()) {
                let mut perms = meta.permissions();
                if perms.readonly() {
                    perms.set_readonly(false);
                    fs::set_permissions(de.path(), perms)?;
                }
            }
            fs::remove_file(de.path())?
        }
    }
    Ok(())
}
