mod arg_driver;
use arg_driver::{ArgDriver, CompilerOptions, CompilerStage, OptimizationLevel};
use crate::lexer::Lexer;
use crate::lexer::token::Token;
use crate::parser::Parser;
use crate::diagnostics::DiagnosticEngine;
use std::io;
use std::fs;
use std::process::Command;

pub struct Driver {
    pub source_file: String,
    pub preprocessor_file: String,
    pub assembly_file: String,
    compiler_options: CompilerOptions
}

impl Driver {
    pub fn new(mut arg_vec: Vec<String>) -> Result<Self, io::Error> {
        // Remove the first argument (it's the name of the compiler itself)
        arg_vec.remove(0);
        let mut arg_driver = ArgDriver::new(arg_vec);
        let compiler_opts = arg_driver.parse()?;

        Ok(Driver {
            source_file: "".to_string(),
            preprocessor_file: "".to_string(),
            assembly_file: "".to_string(),
            compiler_options: compiler_opts
        })
    }

    pub fn run(&mut self, diag_engine: &mut DiagnosticEngine) -> io::Result<()> {
        // Run the Preprocessor first
        // This will become a full-fledged stage in the future
        // REMEMBER: We need to delete the preprocessed file after lexing and parsing!
        // We will loop through all the compiler_options.input_files and perform the pipeline on
        // each one

        for source_file in self.compiler_options.input_files.clone() {
            self.make_file_names(source_file.to_string());
            self.preprocess(&source_file)?;
           
            // Create token_vec so that it won't be out of scope when needed for parsing stage
            let mut token_vec: Vec<Token> = Vec::new();

            // Tokenize
            if self.compiler_options.stop_stage == CompilerStage::Lex {
                let source_text = fs::read_to_string(&self.preprocessor_file)?;
                let lexer: Lexer = Lexer::new(source_text, &self.preprocessor_file);
                token_vec = lexer.tokenize(diag_engine);
            }

            if self.compiler_options.stop_stage == CompilerStage::Parse {
                todo!();
            }
        
            if self.compiler_options.stop_stage == CompilerStage::Assemble {
                // Run the Assemler and Linker
                self.assemble_and_link()?;
            }

            self.cleanup()?;
        }

        Ok(())
    }

    fn make_file_names(&mut self, input_file: String) {
        let mut extension_index: usize = 0;

        // Find the index within the input_file string (starting at the right)
        if let Some(index) = input_file.rfind('.') {
            extension_index = index;
        };

        self.preprocessor_file = input_file[..extension_index].to_string() + ".i";
        self.assembly_file = input_file[..extension_index].to_string() + ".s";
        self.source_file = input_file[..extension_index].to_string();
    }

    fn preprocess(&self, cur_source_file: &str) -> io::Result<()> {
        // Perform the preprocessing stage
        let status = Command::new("gcc")
            .arg("-E")
            .arg("-P")
            .arg(cur_source_file)
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
            .arg(&self.compiler_options.output_file.as_deref().unwrap_or("a.out"))
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


