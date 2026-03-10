use std::io;

#[derive(Debug, PartialEq, Clone)]
pub enum CompilerStage {
    Preprocess,
    Lex,
    Parse,
    Compile,
    Assemble,
    Link
}

#[derive(Debug, PartialEq, Clone)]
pub enum OptimizationLevel {
    Zero,
    O1,
    O2,
    O3
}

#[derive(Debug)]
pub struct CompilerOptions {
    pub input_files: Vec<String>,
    pub output_file: Option<String>,
    pub stop_stage: CompilerStage,
    pub optimization: OptimizationLevel
}

impl CompilerOptions {
    pub fn new() -> Self {
        let input_file_vec: Vec<String> = Vec::new();
        CompilerOptions {
            input_files: input_file_vec,
            output_file: Some("".to_owned()),
            stop_stage: CompilerStage::Link,
            optimization: OptimizationLevel::Zero
        }
    }
}

enum Argument {
    InputFile(String),
    OutputFile(String),
    StopAt(CompilerStage),
    OptLevel(OptimizationLevel),
    Unknown(String)
}

pub struct ArgDriver {
    arg_vec: Vec<String>,
    processed_arg_vec: Vec<Argument>
}

impl ArgDriver {
    pub fn new(args: Vec<String>) -> Self {
        let proc_arg_vec: Vec<Argument> = Vec::new();
        ArgDriver {
            arg_vec: args,
            processed_arg_vec: proc_arg_vec
        }
    }

    pub fn parse(&mut self) -> Result<CompilerOptions, io::Error> {
        let mut iter = self.arg_vec.iter().peekable();
        while let Some(arg) = iter.next() {
            // Check for source files
            if arg.ends_with(".c") {
                self.processed_arg_vec.push(Argument::InputFile(arg.clone()));
                continue;
            } 
            
            if arg.starts_with("--") {
                match arg.as_str() {
                    "--lex" => {
                        self.processed_arg_vec.push(Argument::StopAt(CompilerStage::Lex));
                    }
                    "--parse" => {
                        self.processed_arg_vec.push(Argument::StopAt(CompilerStage::Parse));
                    }
                    "--codegen" => {
                        self.processed_arg_vec.push(Argument::StopAt(CompilerStage::Assemble));
                    }
                    _ => {
                        self.processed_arg_vec.push(Argument::Unknown(arg.clone()));
                    }
                }
                continue;
            }

            if arg.starts_with("-") {
                if arg.starts_with("-O") {
                    match arg.as_str() {
                        "-O1" => {
                            self.processed_arg_vec.push(Argument::OptLevel(OptimizationLevel::O1));
                        }
                        "-O2" => {
                            self.processed_arg_vec.push(Argument::OptLevel(OptimizationLevel::O2));
                        }
                        "-O3" => {
                            self.processed_arg_vec.push(Argument::OptLevel(OptimizationLevel::O3));
                        }
                        _ => {
                            self.processed_arg_vec.push(Argument::Unknown(arg.clone()));
                        }
                    }
                    continue;
                }

                if *arg == "-o" {
                    // We need to peek ahead at the next argument to use it as a output file name
                    match iter.peek() {
                        Some(next) => {
                            self.processed_arg_vec.push(Argument::OutputFile((*next).clone()));
                            iter.next();
                        }
                        None => {
                            return Err(io::Error::new(
                                    io::ErrorKind::InvalidInput,
                                    "'-o' flag requires an output filename"
                            ));
                        }
                    }
                    continue;
                } else {
                    self.processed_arg_vec.push(Argument::Unknown(arg.clone()));
                    continue;
                }
            }
        }

        let mut compiler_options: CompilerOptions = CompilerOptions::new();
        for arg in &self.processed_arg_vec {
            match arg {
                Argument::InputFile(filename) => {
                    compiler_options.input_files.push(filename.clone());
                }
                Argument::OutputFile(filename) => {
                    compiler_options.output_file = Some(filename.clone());
                }
                Argument::StopAt(stage) => {
                    compiler_options.stop_stage = stage.clone();
                }
                Argument::OptLevel(level) => {
                    compiler_options.optimization = level.clone();
                }
                Argument::Unknown(arg) => {
                    return Err(io::Error::new(
                            io::ErrorKind::InvalidInput,
                            format!("Unknown argument: {}", arg)
                    ));
                }
            }
        }
        Ok(compiler_options)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiple_input_files() {
        let arg_vec: Vec<String> = Vec::from(["mdcpl".to_string(), "first.c".to_string(), "second.c".to_string(), "third.c".to_string()]);
        let mut arg_driver: ArgDriver = ArgDriver::new(arg_vec);
        let compiler_options: CompilerOptions = arg_driver.parse().unwrap();
        assert_eq!(compiler_options.input_files.len(), 3);
        assert_eq!(compiler_options.output_file, Some("".to_string()));
        assert_eq!(compiler_options.stop_stage, CompilerStage::Link);
        assert_eq!(compiler_options.optimization, OptimizationLevel::Zero);
    }

    #[test]
    fn test_valid_stop_stage() {
        let mut arg_vec: Vec<String> = Vec::from(["mdcpl".to_string(), "source.c".to_string()]);
        let stage_vec: Vec<CompilerStage> = Vec::from([CompilerStage::Lex, CompilerStage::Parse, CompilerStage::Assemble]);
        let mut stage_str = "".to_string();
        for stage in stage_vec {
            if stage == CompilerStage::Lex {
                stage_str = String::from("--lex");
            }
            if stage == CompilerStage::Parse {
                stage_str = String::from("--parse");
            }
            if stage == CompilerStage::Assemble {
                stage_str = String::from("--codegen")
            }
            arg_vec.push(stage_str.clone());
            let mut arg_driver: ArgDriver = ArgDriver::new(arg_vec.clone());
            let compiler_options: CompilerOptions = arg_driver.parse().unwrap();
            assert_eq!(compiler_options.input_files.len(), 1);
            assert_eq!(compiler_options.output_file, Some("".to_string()));
            assert_eq!(compiler_options.stop_stage, stage);
            assert_eq!(compiler_options.optimization, OptimizationLevel::Zero);
            arg_vec.pop();
        }
    }

    #[test]
    fn test_valid_optimization_levels() {
        let mut arg_vec: Vec<String> = Vec::from(["mdcpl".to_string(), "source.c".to_string()]);
        let optimization_vec: Vec<OptimizationLevel> = Vec::from([OptimizationLevel::O1, OptimizationLevel::O2, OptimizationLevel::O3]);
        let mut opt_string = "".to_string();
        for level in optimization_vec {
            if level == OptimizationLevel::O1 {
                opt_string = String::from("-O1");
            }
            if level == OptimizationLevel::O2 {
                opt_string = String::from("-O2");
            }
            if level == OptimizationLevel::O3 {
                opt_string = String::from("-O3");
            }
            arg_vec.push(opt_string.clone());
            let mut arg_driver: ArgDriver = ArgDriver::new(arg_vec.clone());
            let compiler_options: CompilerOptions = arg_driver.parse().unwrap();
            assert_eq!(compiler_options.input_files.len(), 1);
            assert_eq!(compiler_options.output_file, Some("".to_string()));
            assert_eq!(compiler_options.stop_stage, CompilerStage::Link);
            assert_eq!(compiler_options.optimization, level);
            arg_vec.pop();
        }
    }

    #[test]
    fn test_output_file() {
        let arg_vec: Vec<String> = Vec::from(["mdcpl".to_string(), "source.c".to_string(), "-o".to_string(), "out_source".to_string()]);
        let mut arg_driver: ArgDriver = ArgDriver::new(arg_vec);
        let compiler_options: CompilerOptions = arg_driver.parse().unwrap();
        assert_eq!(compiler_options.input_files.len(), 1);
        assert_eq!(compiler_options.output_file, Some("out_source".to_string()));
        assert_eq!(compiler_options.stop_stage, CompilerStage::Link);
        assert_eq!(compiler_options.optimization, OptimizationLevel::Zero);
    }

    #[test]
    fn test_bad_stage_arg() {
        let arg_vec: Vec<String> = Vec::from(["mdcpl".to_string(), "source.c".to_string(), "--bad-argument".to_string()]);
        let mut arg_driver: ArgDriver = ArgDriver::new(arg_vec);
        let err = arg_driver.parse().unwrap_err();
        assert!(err.to_string().contains("Unknown argument: --bad-argument"));
    }

    #[test]
    fn test_bad_optimization_arg() {
        let arg_vec: Vec<String> = Vec::from(["mdcpl".to_string(), "source.c".to_string(), "-O4".to_string()]);
        let mut arg_driver: ArgDriver = ArgDriver::new(arg_vec);
        let err = arg_driver.parse().unwrap_err();
        assert!(err.to_string().contains("Unknown argument: -O4"));
    }

    #[test]
    fn test_output_file_arg_without_file_name() {
        let arg_vec: Vec<String> = Vec::from(["mdcpl".to_string(), "source.c".to_string(), "-o".to_string()]);
        let mut arg_driver: ArgDriver = ArgDriver::new(arg_vec);
        let err = arg_driver.parse().unwrap_err();
        assert!(err.to_string().contains("'-o' flag requires an output filename"));
    }
}
