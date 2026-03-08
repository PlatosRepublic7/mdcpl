use crate::lexer::Lexer;
use crate::lexer::token::Token;
use crate::diagnostics::DiagnosticEngine;
use std::io;
use std::fs;
use std::process::Command;

pub struct Driver {
    pub source_file: String,
    pub preprocessor_file: String,
    pub assembly_file: String,
    pub output_file: String,
    pub driver_arg_vec: Vec<String>,
    perform_lex: bool,
    perform_parse: bool,
    perform_codegen: bool,
}

impl Driver {
    pub fn new(mut arg_vec: Vec<String>) -> Self {
        let input_file_name = arg_vec.remove(1);
        let file_name_tuple = Driver::make_file_names(input_file_name.clone());
        let driver_args = Driver::make_driver_args(&arg_vec);

        Driver {
            source_file: input_file_name,
            preprocessor_file: file_name_tuple.0,
            assembly_file: file_name_tuple.1,
            output_file: file_name_tuple.2,
            driver_arg_vec: driver_args,
            perform_lex: false,
            perform_parse: false,
            perform_codegen: false,
        }
    }

    pub fn run(&mut self, diag_engine: &mut DiagnosticEngine) -> io::Result<()> {
        // Process arguments first
        self.process_driver_args()?;

        // Run the Preprocessor first
        // This will become a full-fledged stage in the future
        // REMEMBER: We need to delete the preprocessed file after lexing and parsing!
        self.preprocess()?;
       
        // Create token_vec so that it won't be out of scope when needed for parsing stage
        let mut token_vec: Vec<Token> = Vec::new();

        // Tokenize
        if self.perform_lex {
            let source_text = fs::read_to_string(&self.preprocessor_file)?;
            let lexer: Lexer = Lexer::new(source_text, &self.preprocessor_file);
            token_vec = lexer.tokenize(diag_engine);
        }

        if self.perform_parse {
            todo!();
        }

        if self.perform_codegen {
            // Run the Assemler and Linker
            self.assemble_and_link()?;
        }
       
        // Clean up intermediate files
        self.cleanup()?;

        Ok(())
    }

    fn make_file_names(input_file: String) -> (String, String, String) {
        let mut extension_index: usize = 0;

        // Find the index within the input_file string (starting at the right)
        if let Some(index) = input_file.rfind('.') {
            extension_index = index;
        };

        let preprocessor_file: String = input_file[..extension_index].to_string() + ".i";
        let assembly_file: String = input_file[..extension_index].to_string() + ".s";
        let source_file: String = input_file[..extension_index].to_string();
        (preprocessor_file, assembly_file, source_file)
    }

    fn make_driver_args(arg_vec: &[String]) -> Vec<String> {
        // It is assumed here that all the arguments passed to this method will have the following:
        // [program_name(not needed), needed_args...]
        let mut driver_arg_vec: Vec<String> = Vec::new();
        for i in 1..arg_vec.iter().len() {
            driver_arg_vec.push(arg_vec[i].clone());
        }

        driver_arg_vec
    }

    fn process_driver_args(&mut self) -> io::Result<()> {
        for arg in &self.driver_arg_vec {
            match arg.as_str() {
                "--lex" => {
                    self.perform_lex = true;
                    self.perform_parse = false;
                    self.perform_codegen = false;
                }
                "--parse" => {
                    self.perform_lex = true;
                    self.perform_parse = true;
                    self.perform_codegen = false;
                }
                "--codegen" => {
                    self.perform_lex = true;
                    self.perform_parse = true;
                    self.perform_codegen = true;
                }
                _ => {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidInput,
                        "Unrecognized driver argument",
                    ));
                }
            }
        }

        Ok(())
    }

    fn preprocess(&self) -> io::Result<()> {
        // Perform the preprocessing stage
        let status = Command::new("gcc")
            .arg("-E")
            .arg("-P")
            .arg(&self.source_file)
            .arg("-o")
            .arg(&self.preprocessor_file)
            .status()?;

        if !status.success() {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Preprocessor step failed with non-zero exit code",
            ));
        }

        Ok(())
    }

    fn assemble_and_link(&self) -> io::Result<()> {
        // Perform the assembling and linking stage
        let status = Command::new("gcc")
            .arg(&self.assembly_file)
            .arg("-o")
            .arg(&self.output_file)
            .status()?;

        if !status.success() {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Linking setp failed with non-zero exit code",
            ));
        }

        Ok(())
    }

    fn cleanup(&mut self) -> io::Result<()> {
        // Perform cleanup of intermediate files
        fs::remove_file(&self.preprocessor_file)?;

        Ok(())
    }
}
