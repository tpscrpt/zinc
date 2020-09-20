//!
//! The Zinc virtual machine `prove` subcommand.
//!

use std::fs;
use std::path::PathBuf;

use serde_json::Value as JsonValue;
use structopt::StructOpt;

use franklin_crypto::bellman::groth16::Parameters;
use franklin_crypto::bellman::pairing::bn256::Bn256;

use zinc_build::ContractFieldValue as BuildContractFieldValue;
use zinc_build::Program as BuildProgram;
use zinc_build::Value as BuildValue;

use zinc_vm::CircuitFacade;
use zinc_vm::ContractFacade;

use crate::arguments::command::IExecutable;
use crate::error::Error;
use crate::error::IErrorPath;

///
/// The Zinc virtual machine `prove` subcommand.
///
#[derive(Debug, StructOpt)]
#[structopt(name = "prove", about = "Executes circuit and prints program's output")]
pub struct Command {
    /// The path to the binary bytecode file.
    #[structopt(long = "binary", help = "The bytecode file")]
    pub binary_path: PathBuf,

    /// The path to the proving key file.
    #[structopt(long = "proving-key", help = "The proving key file")]
    pub proving_key_path: PathBuf,

    /// The path to the witness JSON file.
    #[structopt(long = "witness", help = "The witness JSON file")]
    pub witness_path: PathBuf,

    /// The path to the public data JSON file.
    #[structopt(long = "public-data", help = "The public data JSON file")]
    pub public_data_path: PathBuf,

    /// The path to the contract storage JSON file.
    #[structopt(long = "storage", help = "The contract storage JSON file")]
    pub storage: Option<PathBuf>,

    /// The method name to call, if the program is a contract.
    #[structopt(long = "method", help = "The method name")]
    pub method: Option<String>,
}

impl IExecutable for Command {
    type Error = Error;

    fn execute(mut self) -> Result<i32, Self::Error> {
        // Read program
        let bytes =
            fs::read(&self.binary_path).error_with_path(|| self.binary_path.to_string_lossy())?;
        let program =
            BuildProgram::try_from_slice(bytes.as_slice()).map_err(Error::ProgramDecoding)?;

        // Read verifying key
        let proving_key_path = self.proving_key_path;
        let file = fs::File::open(&proving_key_path)
            .error_with_path(|| proving_key_path.to_string_lossy())?;
        let params = Parameters::<Bn256>::read(file, true)
            .error_with_path(|| proving_key_path.to_string_lossy())?;

        // Read witness
        let witness_path = self.witness_path;
        let input_template =
            fs::read_to_string(&witness_path).error_with_path(|| witness_path.to_string_lossy())?;
        let input_json = serde_json::from_str(input_template.as_str())?;

        let (output, proof) = match program {
            BuildProgram::Circuit(circuit) => {
                let input_type = circuit.input.clone();
                let input_values = BuildValue::try_from_typed_json(input_json, input_type)?;
                CircuitFacade::new(circuit)
                    .prove::<Bn256>(params, input_values)
                    .map(|(result, proof)| (result, proof))?
            }
            BuildProgram::Contract(contract) => {
                let storage_path = match self.storage.take() {
                    Some(path) => path,
                    None => return Err(Error::ContractStoragePathMissing),
                };

                let storage_str = fs::read_to_string(&storage_path)
                    .error_with_path(|| witness_path.to_string_lossy())?;
                let storage_json = serde_json::from_str(storage_str.as_str())?;
                let storage_values = match storage_json {
                    JsonValue::Array(array) => {
                        let mut storage_values = Vec::with_capacity(contract.storage.len());
                        for (field, value) in contract.storage.clone().into_iter().zip(array) {
                            storage_values.push(BuildContractFieldValue::new(
                                field.name,
                                BuildValue::try_from_typed_json(value, field.r#type)?,
                                field.is_public,
                                field.is_external,
                            ));
                        }
                        storage_values
                    }
                    value => return Err(Error::InvalidContractStorageFormat { found: value }),
                };

                let method_name = self.method.ok_or(Error::MethodNameNotFound)?;
                let method = contract.methods.get(method_name.as_str()).cloned().ok_or(
                    Error::MethodNotFound {
                        name: method_name.clone(),
                    },
                )?;
                let input_values = BuildValue::try_from_typed_json(input_json, method.input)?;
                ContractFacade::new(contract)
                    .prove::<Bn256>(
                        params,
                        input_values,
                        BuildValue::Contract(storage_values),
                        method_name,
                    )
                    .map(|(result, proof)| (result, proof))?
            }
        };

        // Write pubdata
        let pubdata_json = serde_json::to_string_pretty(&output.into_json())? + "\n";
        let public_data_path = self.public_data_path;
        fs::write(&public_data_path, &pubdata_json)
            .error_with_path(|| public_data_path.to_string_lossy())?;

        // Write proof to stdout
        let mut proof_bytes = Vec::new();
        proof.write(&mut proof_bytes).expect("writing to vec");
        let proof_hex = hex::encode(proof_bytes);
        println!("{}", proof_hex);

        Ok(zinc_const::exit_code::SUCCESS as i32)
    }
}
