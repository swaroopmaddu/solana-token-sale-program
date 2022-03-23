use solana_program::{
    program_error::ProgramError,
    program_pack::{IsInitialized, Pack, Sealed},
    pubkey::Pubkey,
};

use arrayref::{array_mut_ref, array_ref, array_refs, mut_array_refs};

pub struct TokenSaleProgramData {
    pub is_initialized: bool,
    pub seller_pubkey: Pubkey,
    pub temp_token_account_pubkey: Pubkey,
    pub per_token_price: u64,
}

impl TokenSaleProgramData {
    pub fn init(
        &mut self,
        is_initialized: bool,              // 1
        seller_pubkey: Pubkey,              // 32
        temp_token_account_pubkey: Pubkey, // 32
        per_token_price: u64,              // 8
    ) {
        self.is_initialized = is_initialized;
        self.seller_pubkey = seller_pubkey;
        self.temp_token_account_pubkey = temp_token_account_pubkey;
        self.per_token_price = per_token_price;
    }
}

impl Sealed for TokenSaleProgramData {}

impl IsInitialized for TokenSaleProgramData {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl Pack for TokenSaleProgramData {
    const LEN: usize = 73; // 1 + 32 + 32 + 8 
    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, TokenSaleProgramData::LEN];
        let (
            is_initialized,
            seller_pubkey,
            temp_token_account_pubkey,
            per_token_price,
        ) = array_refs![src, 1, 32, 32, 8]; // 각 변수에 알맞은 바이트대로 잘라서 넣어줌

        let is_initialized = match is_initialized {
            [0] => false,
            [1] => true,
            _ => return Err(ProgramError::InvalidAccountData),
        };

        return Ok(TokenSaleProgramData {
            is_initialized,
            seller_pubkey: Pubkey::new_from_array(*seller_pubkey),
            temp_token_account_pubkey: Pubkey::new_from_array(*temp_token_account_pubkey),
            per_token_price: u64::from_le_bytes(*per_token_price),
        });
    }

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, TokenSaleProgramData::LEN];
        let (
            is_initialized_dst,
            seller_pubkey_dst,
            temp_token_account_pubkey_dst,
            per_token_price_dst
        ) = mut_array_refs![dst, 1, 32, 32, 8];

        let TokenSaleProgramData {
            is_initialized,
            seller_pubkey,
            temp_token_account_pubkey,
            per_token_price,
        } = self;

        is_initialized_dst[0] = *is_initialized as u8;
        seller_pubkey_dst.copy_from_slice(seller_pubkey.as_ref());
        temp_token_account_pubkey_dst.copy_from_slice(temp_token_account_pubkey.as_ref());
        *per_token_price_dst = per_token_price.to_le_bytes();
    }
}
