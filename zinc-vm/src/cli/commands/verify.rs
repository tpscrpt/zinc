use structopt::StructOpt;
use std::path::PathBuf;
use crate::Error;
use std::fs;
use franklin_crypto::bellman::groth16::{Parameters, Proof};
use colored::Colorize;
use std::process::exit;
use crate::witness::json_to_flat_input;
use pairing::bn256::Bn256;

#[derive(Debug, StructOpt)]
pub struct VerifyCommand {
    #[structopt(short = "c", long = "circuit", about = "Circuit's bytecode file")]
    pub circuit_file: PathBuf,

    #[structopt(short = "P", long = "params", about = "Circuit's bytecode file")]
    pub params_path: PathBuf,

    #[structopt(short = "p", long = "proof", about = "Proof file")]
    pub proof_path: PathBuf,

    #[structopt(short = "o", long = "output", about = "Program's output file")]
    pub output_path: PathBuf
}

impl VerifyCommand {
    pub fn execute(&self) -> Result<(), Error> {
        let params_file = fs::File::open(&self.params_path)?;
        let params = Parameters::<Bn256>::read(params_file, true)?;

        let proof_file = fs::File::open(&self.proof_path)?;
        let proof = Proof::<Bn256>::read(proof_file)?;

        let output_text = fs::read_to_string(&self.output_path)?;
        let output_json = json::parse(output_text.as_str())?;
        // TODO: Remove unwrap
        let output = json_to_flat_input(&output_json).unwrap();

        let verified = zinc_vm::verify(&params, &proof, &output)?;

        if verified {
            println!("{}", "Ok".bold().green());
        } else {
            println!("{}", "Verification failed".bold().red());
            exit(1);
        }

        Ok(())
    }
}
