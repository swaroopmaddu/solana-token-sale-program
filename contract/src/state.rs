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
    pub swap_sol_amount: u64,
    pub swap_token_amount: u64,
}

impl TokenSaleProgramData {
    pub fn init(
        &mut self,
        is_initialized: bool,              // 1
        seller_pubkey: Pubkey,              // 32
        temp_token_account_pubkey: Pubkey, // 32
        swap_sol_amount: u64,              // 8
        swap_token_amount: u64,            // 8
    ) {
        self.is_initialized = is_initialized;
        self.seller_pubkey = seller_pubkey;
        self.temp_token_account_pubkey = temp_token_account_pubkey;
        self.swap_sol_amount = swap_sol_amount;
        self.swap_token_amount = swap_token_amount;
    }
}

impl Sealed for TokenSaleProgramData {}

impl IsInitialized for TokenSaleProgramData {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl Pack for TokenSaleProgramData {
    const LEN: usize = 81; // 1 + 32 + 32 + 8 + 8
    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, TokenSaleProgramData::LEN];
        let (
            is_initialized,
            seller_pubkey,
            temp_token_account_pubkey,
            swap_sol_amount,
            swap_token_amount,
        ) = array_refs![src, 1, 32, 32, 8, 8]; // 각 변수에 알맞은 바이트대로 잘라서 넣어줌

        let is_initialized = match is_initialized {
            [0] => false,
            [1] => true,
            _ => return Err(ProgramError::InvalidAccountData),
        };

        return Ok(TokenSaleProgramData {
            is_initialized,
            seller_pubkey: Pubkey::new_from_array(*seller_pubkey),
            temp_token_account_pubkey: Pubkey::new_from_array(*temp_token_account_pubkey),
            swap_sol_amount: u64::from_le_bytes(*swap_sol_amount),
            swap_token_amount: u64::from_le_bytes(*swap_token_amount),
        });
    }

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, TokenSaleProgramData::LEN];
        let (
            is_initialized_dst,
            seller_pubkey_dst,
            temp_token_account_pubkey_dst,
            swap_sol_amount_dst,
            swap_token_amount_dst,
        ) = mut_array_refs![dst, 1, 32, 32, 8, 8];

        let TokenSaleProgramData {
            is_initialized,
            seller_pubkey,
            temp_token_account_pubkey,
            swap_sol_amount,
            swap_token_amount,
        } = self;

        is_initialized_dst[0] = *is_initialized as u8;
        seller_pubkey_dst.copy_from_slice(seller_pubkey.as_ref());
        temp_token_account_pubkey_dst.copy_from_slice(temp_token_account_pubkey.as_ref());
        *swap_sol_amount_dst = swap_sol_amount.to_le_bytes();
        *swap_token_amount_dst = swap_token_amount.to_le_bytes();
    }
}
