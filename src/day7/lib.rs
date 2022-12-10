use std::fs;

type FileSystemItemId = usize;
struct FileSystem {
    data: Vec<FileSystemItem>,
    current_dir_id: FileSystemItemId,
}

impl FileSystem {
    fn new() -> Self {
        Self { 
            data: vec![
                FileSystemItem {
                    name: String::from("/"),
                    parent: None,
                    size: 0,
                    children: Some(Vec::new()),
                }
            ], 
            current_dir_id: 0 
        }
    }

    fn change_dir(&mut self, dir: &str) {
        let current_dir = self.get_current_dir();

        // Pop up folder
        if dir == ".." {
            self.current_dir_id = current_dir.parent.expect("There is no parent");
            return;
        }

        // Move to folder
        let mut dir_id = None;
        let children = current_dir.children.as_ref().expect("Current dir is incorrectly pointing to a file");

        for &child_id in children {
            let child = self.get_item(child_id);
            
            // Check it is a folder and then match name
            if child.children.is_some() && child.name == dir {
                dir_id = Some(child_id);
                break;
            }
        }

        match dir_id {
            Some(dir_id) => {
                self.current_dir_id = dir_id;
            }
            None => panic!("Folder not found in the current dir"),
        }
    }

    fn add_file(&mut self, name: &str, size: u32) -> FileSystemItemId {
        let new_file = FileSystemItem {
            name: String::from(name),
            parent: Some(self.current_dir_id),
            size,
            children: None,
        };
        let new_file_id = self.data.len();
        self.data.push(new_file);
        
        let mut current_dir = self.get_mut_current_dir();
        current_dir.children.as_mut().expect("Invalid folder").push(new_file_id);
        current_dir.size += size;

        // Back propagate
        while current_dir.parent.is_some() {
            let current_dir_parent = current_dir.parent.unwrap();

            current_dir = self.get_mut_item(current_dir_parent);

            current_dir.size += size;
        }

        new_file_id
    }

    fn add_folder(&mut self, name: &str) -> FileSystemItemId {
        let new_folder = FileSystemItem {
            name: String::from(name),
            parent: Some(self.current_dir_id),
            size: 0,
            children: Some(Vec::new()),
        };
        let new_folder_id = self.data.len();
        self.data.push(new_folder);

        let current_dir = self.get_mut_current_dir();
        current_dir.children.as_mut().expect("Invalid folder").push(new_folder_id);

        new_folder_id
    }

    #[inline]
    fn get_current_dir(&self) -> &FileSystemItem {
        self.data.get(self.current_dir_id).expect("Current dir is not valid")
    }

    #[inline]
    fn get_mut_current_dir(&mut self) -> &mut FileSystemItem {
        self.data.get_mut(self.current_dir_id).expect("Current dir is not valid")
    }

    #[inline]
    fn get_item(&self, id: FileSystemItemId) -> &FileSystemItem {
        self.data.get(id).expect("Dir id is not valid")
    }

    #[inline]
    fn get_mut_item(&mut self, id: FileSystemItemId) -> &mut FileSystemItem {
        self.data.get_mut(id).expect("Dir id is not valid")
    }

    fn simple_total_size(&self, threshold: u32) -> u32 {
        self.data
        .iter()
        .filter(|item| item.children.is_some())
        .filter(|dir| dir.size < threshold)
        .fold(0, |acc, item| acc + item.size )
    }

    fn folder_removal_selection(&self, disk_space: u32, space_needed: u32) -> u32 {
        // Root dir
        let used_space = self.data[0].size;
        let free_space = disk_space - used_space;
        let missing_space = space_needed - free_space;

        self.data
        .iter()
        .filter(|item| item.children.is_some())
        .map(|dir| dir.size)
        .filter(|dir| dir >= &missing_space)
        .min()
        .unwrap()
    }
}

struct FileSystemItem {
    name: String,
    parent: Option<FileSystemItemId>,
    size: u32,
    children: Option<Vec<FileSystemItemId>>,
}

pub struct Day7 {}

impl Day7 {
    pub fn run() {
        let filename = "src/day7/input.txt";
        let data = fs::read_to_string(filename).unwrap();

        // Part 1
        let mut filesystem = FileSystem::new();

        // Dir '/' is already the entry point
        for line in data.lines().skip(1) {
            if line.starts_with("$ cd") {
                let dir = line.split_whitespace().nth(2).unwrap();
                
                filesystem.change_dir(dir);
            } else if line == "$ ls" {
                continue;
            } else if line.starts_with("dir") {
                let folder_name = line.chars().skip(4).collect::<String>();
                
                filesystem.add_folder(&folder_name);
            } else {
                let data: Vec<&str> = line.split_whitespace().take(2).collect();
                let file_size = data[0].parse().unwrap();
                let file_name = data[1];

                filesystem.add_file(file_name, file_size);
            }
        }

        println!("Part 1 - Total size is: {}", filesystem.simple_total_size(100_000));

        println!("Part 2 - Size of folder to remove: {}", filesystem.folder_removal_selection(70_000_000, 30_000_000));

        // TODO: Go through the folder
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn generate_filesystem() -> FileSystem {
        let mut filesystem = FileSystem::new();

        filesystem.add_folder("a");
        filesystem.add_file("b", 14848514);
        filesystem.add_file("c", 8504156);
        filesystem.add_folder("d");

        filesystem.change_dir("a");

        filesystem.add_folder("e");
        filesystem.add_file("f", 29116);
        filesystem.add_file("g", 2557);
        filesystem.add_file("h.lst", 62596);

        filesystem.change_dir("e");
        filesystem.add_file("i", 584);
        
        filesystem.change_dir("..");
        filesystem.change_dir("..");
        filesystem.change_dir("d");

        filesystem.add_file("j", 4060174);
        filesystem.add_file("d.log", 8033020);
        filesystem.add_file("d.ext", 5626152);
        filesystem.add_file("k", 7214296);

        filesystem.change_dir("..");

        filesystem
    }

    #[test]
    fn test_dir_sizes() {
        let mut filesystem = generate_filesystem();
        assert_eq!(filesystem.get_current_dir().name, "/");

        filesystem.change_dir("a");
        assert_eq!(filesystem.get_current_dir().size, 94853);

        filesystem.change_dir("e");
        assert_eq!(filesystem.get_current_dir().size, 584);
        
        filesystem.change_dir("..");    
        filesystem.change_dir("..");
        filesystem.change_dir("d");
        assert_eq!(filesystem.get_current_dir().size, 24933642);

        filesystem.change_dir("..");
        assert_eq!(filesystem.get_current_dir().size, 48381165);
    }

    #[test]
    fn test_total_size() {
        let filesystem = generate_filesystem();
        assert_eq!(filesystem.simple_total_size(100_000), 95437);
    }

    #[test]
    fn test_smallest_free_space() {
        let filesystem = generate_filesystem();
        assert_eq!(filesystem.folder_removal_selection(70_000_000, 30_000_000), 24933642);
    }
}