use std::fs::{File, OpenOptions};
use std::io;
use std::io::Write;

// 20.3.路径
fn main() {
    //region 20.3.路径
    println!("\n\n=====20.3.路径=====");
    // Path 结构体代表了底层文件系统的文件路径。
    // Path 分两种： posix:Path，针对 Unix
    //              windows::Path
    // Path 可以从 OsStr 类型创建，且提供数种方法，用于获取路径指向的文件/目录的信息
    // 注意：Path 在内部并不是用 UTF-8 字符串表示的，而是存储为若干字节(Vec<u8>)的 Vector。
    // 因此，将 Path 转化成 &str 并非零开销，且可能失败，因此其返回一个 Option.
    use std::path::Path;
    // 从 `&'static str` 创建一个 `Path`。
    let path = Path::new(".");
    // `display` 方法返回一个可显示（showable）的结构体
    let display = path.display();
    // `join` 使用操作系统特定的分隔符来合并路径到一个字节窗口，并返回新的路径
    let new_path = path.join("a").join("b");
    // 将路径转换成一个字符串切片
    match new_path.to_str() {
        None => panic!("new path is not a valid UTF-8 sequence"),
        Some(s) => println!("new path is {}", s),
    }
    println!("display: {}", display);
    //endregion

    //region 20.4.文件输入输出(I/O)
    println!("\n\n=====20.4.文件输入输出(I/O)=====");
    // File 结构体表示一个被打开的文件（包裹了一个文件描述符），并赋予了对所表示的文件的读写能力。
    // 由于在进行文件 I/O 操作时可能出现各种错误，因此 File 的所有方法都返回 io::Result<T> 类型
    // 它是 Result<T, io::Error> 的别名
    // 这使得所有的 I/O 操作的失败都变成显式的了，借助于此，程序员可以看到所有的失败路径，并被鼓励
    // 主动地处理这些情形。
    //endregion

    //region 20.4.1.打开文件
    println!("\n\n=====20.4.1.打开文件=====");
    // open 静态方法能够以只读模式(read-only mode)打开一个文件
    // File 拥有资源，即文件描述符(file descriptor)，它会在自身被 drop 时关闭文件。
    use std::fs::File;
    use std::io::prelude::*;
    // 创建指向所需的文件的路径
    let path = Path::new("src/main.rs");
    let display = path.display();

    // 以只读方式打开路径，返回 `io::Result<File>`
    let mut file = match File::open(&path) {
        // `io::Error` 的 `description` 方法返回一个描述错误的字符串
        Err(why) => panic!("couldn't open {}: {:?}", display, why),
        Ok(file) => file,
    };
    // 读取文件内容到一个字符串，返回 `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {:?}", display, why),
        Ok(_) => print!("{} contents:\n{}", display, s),
    }
    //endregion

    //region 20.4.2.创建文件create
    println!("\n\n=====20.4.2.创建文件create=====");
    // create 静态方法以只写模式(write-only mode)打开一个文件，若文件已经存在，则旧内容被销毁
    // 否则，将创建一个新文件
    static LOREM_IPSUM: &'static str =
        "Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam,
        quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo
        consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse
        cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non
        proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
        ";
    let path = Path::new("out/lorem_ipsum.txt");
    let display = path.display();
    // 以只写模式打开文件，返回 `io::Result<File>`
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create file: {}: {:?}", display, why),
        Ok(file) => file,
    };
    // 将 `LOREM_IPSM` 字符串写进 `file`，返回 `io::Result<()>`
    match file.write_all(LOREM_IPSUM.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {:?}", display, why),
        Ok(_) => println!("successfully wrote {}", display),
    }
    //endregion

    //region 20.4.3.读取行
    println!("\n\n=====20.4.3.读取行=====");
    // 方法 lines() 在文件的行上返回一个迭代器
    // File::open 需要一个泛型 AsRef<Path>，这正是 read_lines() 期望的输入。
    use std::io::{self, BufRead};
    if let Ok(lines) = read_lines("src/main.rs") {
        for line in lines {
            if let Ok(content) = line {
                println!("{}", content);
            }
        }
    }

    fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
        where P: AsRef<Path>, {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    }
    //endregion

    //region 20.6.文件系统操作
    println!("\n\n=====20.6.文件系统操作=====");
    use std::fs;
    use std::fs::OpenOptions;
    use std::os::unix;
    use std::io::prelude::*;
    // `% cat path` 的简单实现
    fn cat(path: &Path) -> io::Result<String> {
        let mut f = File::open(path)?;
        let mut s = String::new();
        match f.read_to_string(&mut s) {
            Ok(_) => Ok(s),
            Err(e) => Err(e),
        }
    }
    // `% echo s > path`的简单实现
    fn echo(s: &str, path: &Path) -> io::Result<()> {
        let mut f = File::create(path)?;
        f.write_all(s.as_bytes())
    }
    // `% touch path`
    fn touch(path: &Path) -> io::Result<()> {
        match OpenOptions::new().create(true).write(true).open(path) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    println!("mkdir a");
    match fs::create_dir("a") {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(_) => {},
    }
    println!("`echo hello > a/b.txt`") ;
    echo("hello", &Path::new("a/b.txt")).unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
    });

    println!("`mkdir -p a/c/d`");
   // 递归地创建一个目录，返回 `io::Result<()>`
    fs::create_dir_all("a/c/d").unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
    });

    println!("`touch a/c/e.txt`");
    touch(&Path::new("a/c/e.txt")).unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
    });

    println!("`ln -s ../b.txt a/c/b.txt`");
    // 创建一个符号链接，返回 `io::Result<()>`
    if cfg!(target_family = "unix") {
        unix::fs::symlink("../b.txt", "a/c/b.txt").unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
        });
    }

    println!("`cat a/c/b.txt`");
    match cat(&Path::new("a/c/b.txt")) {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(s) => println!("> {}", s),
    }

    println!("`ls a`");
    // 读取目录的内容，返回 `io::Result<Vec<Path>>`
    match fs::read_dir("a") {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(paths) => for path in paths {
            println!("> {:?}", path.unwrap().path());
        },
    }

    println!("`rm a/c/e.txt`");
    // 删除一个文件，返回 `io::Result<()>`
    fs::remove_file("a/c/e.txt").unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
    });

    println!("`rmdir a/c/d`");
    // 移除一个空目录，返回 `io::Result<()>`
    fs::remove_dir("a/c/d").unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
    });

























}
