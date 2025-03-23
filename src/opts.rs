use clap::Parser;
use std::path::Path;

// Debug：让 Opts 结构体可以用 println!("{:?}", opts); 进行调试打印。
// 让data structure 与命令行的参数关联起来
#[derive(Debug, Parser)]
// 定义 CLI 程序的元信息
// name: 命令行程序的名称（CLI 名称）
// version:自动读取 Cargo.toml 的 package.version 作为 CLI 版本。
// author: 读取 Cargo.toml 的 package.authors 作为作者信息。
// about:CLI 程序的简短描述，通常显示在 --help 中。
// long_about:可选的详细描述（如果有），None 表示没有额外的说明。
#[command(name = "rcli", version, author, about, long_about = None)] // 为这个cli或者parser加一些额外的信息
pub struct Opts {
    // 这表示 Opts 结构体支持子命令，所有子命令的定义都会放在 SubCommand 结构体中。
    // cmd: SubCommand 是 CLI 解析后的子命令字段。
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(name = "csv", about = "Show CSV or convert Csv to other formats")]
    Csv(CsvOpts),
}

#[derive(Debug, Parser)]
// 定义了一个结构体 CsvOpts，用来存储与 csv 子命令相关的所有选项。
pub struct CsvOpts {
    // 使用 verify_input_file 函数验证输入路径的有效性，确保提供的路径是有效的文件路径。
    #[arg(short, long, value_parser = verify_input_file)]
    pub input: String,
    // 如果未在命令行中提供输出文件参数，则默认为 "output.json"。
    // default_value_t会调用"output.json".into()实现&str转换为String
    #[arg(short, long, default_value = "output.json")]
    pub output: String,
    // default_value_t与default_value区别:default_value_t给引用类型,而default_value实现了把值convert成下面的类型
    // default_value_t = ','：如果未提供分隔符，则默认为 ','，即使用逗号作为分隔符。
    #[arg(short, long, default_value_t = ',')]
    // delimiter 字段用于指定 CSV 文件中的分隔符。
    pub delimiter: char,
    // 如果未提供该选项，默认为 true，即假设 CSV 文件有头部。
    #[arg(long, default_value_t = true)]
    // header 字段用于指定 CSV 文件是否包含头部（即列名）。
    pub header: bool,
}

// Result<String, &'static str> 同等于 Result<String, String> + "File does not exist".into,少了一层转换
fn verify_input_file(filename: &str) -> Result<String, &'static str> {
    // Alt + enter 弹出quick fix
    if Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("File does not exist")
    }
}
