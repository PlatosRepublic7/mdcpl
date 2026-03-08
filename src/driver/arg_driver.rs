use std::io;

#[derive(PartialEq, Clone)]
pub enum CompilerStage {
    Preprocess,
    Lex,
    Parse,
    Compile,
    Assemble,
    Link
}

#[derive(PartialEq, Clone)]
pub enum OptimizationLevel {
    Zero,
    O1,
    O2,
    O3
}

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

