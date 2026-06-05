#[allow(unused)]
use hex::{decode, encode};

pub fn decode_hex(hex_str: &str) -> Result<Vec<u8>, String> {
    //Decode hex string into Vec<u8>, return error string on failure

    let transaction_bytes = hex::decode(hex_str);

    match transaction_bytes {
        Ok(bytes) => Ok(bytes),
        Err(error) => Err(error.to_string()),
    }
}

pub fn to_big_endian(bytes: &[u8]) -> Vec<u8> {
    //Reverse the byte order of input slice and return as Vec<u8>

    let mut result = Vec::new();

    for i in (0..bytes.len()).rev() {
        result.push(bytes[i]);
    }
    result
}

pub fn bytes_to_hex(bytes: &[u8]) -> String {
    //Implement conversion of bytes slice to hex string

    hex::encode(bytes)
    
}

pub fn hex_to_bytes(hex: &str) -> Result<Vec<u8>, hex::FromHexError> {
    //Implement conversion of hex string to bytes vector

    hex::decode(hex)
    
}

pub fn swap_endian_u32(num: u32) -> [u8; 4] {
    //Implement little-endian byte swap for u32
    num.to_le_bytes()
}

pub fn parse_satoshis(input: &str) -> Result<u64, String> {
    //Parse input string to u64, return error string if invalid

    let result = input.parse();
    match result {
        Ok(num) => Ok(num),
        Err(_) => Err("Invalid satoshi amount".to_string()),
    }
}

pub enum ScriptType {
    P2PKH,
    P2WPKH,
    Unknown,
}

pub fn classify_script(script: &[u8]) -> ScriptType {
    //Match script pattern and return corresponding ScriptType
    match script {
        [0x76, 0xA9, ..] => ScriptType::P2PKH,
        [0x00, 0x14, ..] => ScriptType::P2WPKH,
        _ => ScriptType::Unknown,
    }
}

// TODO: complete Outpoint tuple struct
pub struct Outpoint(pub String, pub u32); //TXID, vout

pub fn read_pushdata(script: &[u8]) -> &[u8] {
    //Return the pushdata portion of the script slice (assumes pushdata starts at index 2)
    &script[2..]
}

pub trait Wallet {
    fn balance(&self) -> u64;
}

pub struct TestWallet {
    pub confirmed: u64,
}

impl Wallet for TestWallet {
    fn balance(self: &TestWallet) -> u64 {
        //Return the wallet's confirmed balance
        self.confirmed
    }
}

pub fn apply_fee(balance: &mut u64, fee: u64) {
    // Subtract fee from mutable balance reference
    let mutable_balance = *balance;
    let result = mutable_balance - fee;
    *balance = result;
}

pub fn move_txid(txid: String) -> String {
    //Return formatted string including the txid for display or logging
    format!("txid: {}", txid)
}

// TODO: Add necessary derive traits
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Opcode {
    OpChecksig,
    OpDup,
    OpInvalid,
}

impl Opcode {
    pub fn from_byte(byte: u8) -> Result<Self, String> {
        //Implement mapping from byte to Opcode variant
        match byte {
            0xac => Ok(Opcode::OpChecksig),
            0x76 => Ok(Opcode::OpDup),
            _ => Err("Invalid opcode: 0x00".to_string()),
        }
    }
}

// TODO: Add necessary derive traits
#[derive(Debug, Clone, PartialEq)]
pub struct UTXO {
    pub txid: Vec<u8>,
    pub vout: u32, //output index of that transaction
    pub value: u64,
}

pub fn consume_utxo(utxo: UTXO) -> UTXO {
    // Implement UTXO consumption logic (if any)
    utxo
}
