use std::fs;
use std::path::Path;

// CK3目录ID
const STEAM_CRUSADER_KINGS_DIR_ID: &str = "1158310";
// 集成汉化目录ID
const STEAM_WORKSHOP_DIR_ID: &str = "2903983628";
// 中文本地化文件名
const CHINESE_FILE_NAME: &str = "l_simp_chinese";

fn main() {
    change_directory_to_steam_crusader_kings_workshop_folder();

    let target_dir = Path::new(STEAM_WORKSHOP_DIR_ID)
        .canonicalize()
        .expect("获取规范路径时发生错误");

    let target_files = get_files_with_name(&target_dir, CHINESE_FILE_NAME);

    let parent_dir = Path::new(".");
    for entry in fs::read_dir(parent_dir).unwrap() {
        let path = entry.unwrap().path().canonicalize().unwrap();

        if path.is_dir() && path != target_dir {
            delete_files_in_dir(&path, &target_files);
        }
    }
}

fn get_files_with_name(dir: &Path, name: &str) -> Vec<String> {
    let mut result = Vec::new();
    if dir.is_dir() {
        for entry in fs::read_dir(dir).unwrap() {
            if let Ok(entry) = entry {
                let path = entry.path();
                let file_name_str = path.file_name().unwrap().to_str().unwrap();
                if path.is_file() && file_name_str.contains(name) {
                    result.push(file_name_str.to_string());
                } else if path.is_dir() {
                    result.append(&mut get_files_with_name(&path, name));
                }
            }
        }
    }
    result
}

fn delete_files_in_dir(dir: &Path, target_files: &[String]) {
    if dir.is_dir() {
        for entry in fs::read_dir(dir).unwrap() {
            if let Ok(entry) = entry {
                let path = entry.path();
                let file_name_str = path.file_name().unwrap().to_str().unwrap();
                if path.is_file()
                    && target_files
                        .iter()
                        .any(|target_file| target_file == file_name_str)
                {
                    fs::remove_file(&path).unwrap();
                    // println!("删除文件：{}", path.display());
                } else if path.is_dir() {
                    delete_files_in_dir(&path, target_files);
                }
            }
        }
    }
}

fn change_directory_to_steam_crusader_kings_workshop_folder() {
    let current_dir = Path::new(".")
        .canonicalize()
        .expect("获取规范路径时发生错误");

    if current_dir.file_name().unwrap().to_str().unwrap() != STEAM_CRUSADER_KINGS_DIR_ID {
        let has_steam_crusader_kings_dir = fs::read_dir(&current_dir).unwrap().any(|entry| {
            let path = entry.unwrap().path();
            path.is_dir()
                && path.file_name().unwrap().to_str().unwrap() == STEAM_CRUSADER_KINGS_DIR_ID
        });

        if !has_steam_crusader_kings_dir {
            panic!(
                "当前目录不是 {}，也没有 {} 目录",
                STEAM_CRUSADER_KINGS_DIR_ID, STEAM_CRUSADER_KINGS_DIR_ID
            );
        }

        std::env::set_current_dir(STEAM_CRUSADER_KINGS_DIR_ID).unwrap();
    }
}

