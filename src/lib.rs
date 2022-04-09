use astro_format::{decode, encode};
use fides::{ed25519, merkle_root};
use opis::Int;
use std::error::Error;

#[derive(Clone, Debug)]
pub struct Transaction {
    pub chain: Int,
    pub counter: Int,
    pub recipient: [u8; 32],
    pub sender: [u8; 32],
    pub signature: [u8; 64],
    pub solar_limit: Int,
    pub solar_price: Int,
    pub value: Int,
}

impl Transaction {

    pub fn new() -> Self {
        Transaction {
            chain: Int::zero(),
            counter: Int::zero(),
            recipient: [0_u8; 32],
            sender: [0_u8; 32],
            signature: [0_u8; 64],
            solar_limit: Int::zero(),
            solar_price: Int::zero(),
            value: Int::zero()
        }
    }

    pub fn body_hash(&self) -> [u8; 32] {
        merkle_root(&vec![
            self.chain.to_bytes(),
            self.counter.to_bytes(),
            self.recipient.to_vec(),
            self.sender.to_vec(),
            self.solar_limit.to_bytes(),
            self.solar_price.to_bytes(),
            self.value.to_bytes()
        ])
    }
    
    pub fn hash(&self) -> [u8; 32] {
        merkle_root(&vec![
            self.body_hash().to_vec(),
            self.signature.to_vec()
        ])
    }

    pub fn from_bytes(arg: &Vec<u8>) -> Result<Self, Box<dyn Error>> {

        let set = decode(&arg);

        if set.len() == 8 {
            
            let tx = Transaction {
                chain: Int::from_bytes(&set[0]),
                counter: Int::from_bytes(&set[1]),
                recipient: set[2].clone().try_into().unwrap_or(Err("Recipient error!")?),
                sender: set[3].clone().try_into().unwrap_or(Err("Sender error!")?),
                signature: set[4].clone().try_into().unwrap_or(Err("Signature error!")?),
                solar_limit: Int::from_bytes(&set[5]),
                solar_price: Int::from_bytes(&set[6]),
                value: Int::from_bytes(&set[7])
            };

            Ok(tx)

        } else {
            Err("Parameters error!")?
        }

    }

    pub fn to_bytes(&self) -> Vec<u8> {
        
        encode(&vec![
            self.chain.to_bytes(),
            self.counter.to_bytes(),
            self.recipient.to_vec(),
            self.sender.to_vec(),
            self.signature.to_vec(),
            self.solar_limit.to_bytes(),
            self.solar_price.to_bytes(),
            self.value.to_bytes()
        ])

    }

    pub fn verify(&self) -> bool {
        ed25519::verify(&self.body_hash(), &self.sender, &self.signature)
    }

}
