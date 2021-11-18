use solana_program::{account_info::{next_account_info, AccountInfo}, entrypoint::ProgramResult, msg, native_token::LAMPORTS_PER_SOL, program::{invoke, invoke_signed}, program_error::ProgramError, program_pack::Pack, pubkey::Pubkey, system_instruction, sysvar::{rent::Rent, Sysvar}};

use crate::{instruction::TokenSaleInstruction, state::TokenSaleProgramData};
pub struct Processor;
impl Processor {
    pub fn process(
        token_program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction = TokenSaleInstruction::unpack(instruction_data)?;

        match instruction {
            TokenSaleInstruction::InitTokenSale {
                swap_sol_amount,
                swap_token_amount,
            } => {
                msg!("Instruction: init token sale program");
                Self::init_token_sale_program(
                    accounts,
                    swap_sol_amount,
                    swap_token_amount,
                    token_program_id,
                )
            }
            TokenSaleInstruction::BuyToken {} => {
                msg!("Instruction: buy token");
                Self::buy_token(accounts, token_program_id)
            }
        }
    }

    //seller account info 
    //temp token account - TokenAccount isolated by the amount of tokens to be sold
    //token sale program account info - Save the data about token sale
    //rent - To check if the rent fee is exempted
    //token program - To change the owner of temp token account to token sale program

    fn init_token_sale_program(
        account_info_list: &[AccountInfo],
        swap_sol_amount: u64,
        swap_token_amount: u64,
        token_sale_program_id: &Pubkey,
    ) -> ProgramResult {
        let account_info_iter = &mut account_info_list.iter();

        let seller_account_info = next_account_info(account_info_iter)?;
        if !seller_account_info.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        let temp_token_account_info = next_account_info(account_info_iter)?;
        if *temp_token_account_info.owner != spl_token::id() {
            return Err(ProgramError::IncorrectProgramId);
        }

        let token_sale_program_account_info = next_account_info(account_info_iter)?;

        let rent_account_info = next_account_info(account_info_iter)?;
        let rent = &Rent::from_account_info(rent_account_info)?;

        if !rent.is_exempt(
            token_sale_program_account_info.lamports(),
            token_sale_program_account_info.data_len(),
        ) {
            return Err(ProgramError::AccountNotRentExempt);
        }

        //get data from account (needed `is_writable = true` option)
        let mut token_sale_program_account_data = TokenSaleProgramData::unpack_unchecked(
            &token_sale_program_account_info.try_borrow_data()?,
        )?;
        if token_sale_program_account_data.is_initialized {
            return Err(ProgramError::AccountAlreadyInitialized);
        }

        token_sale_program_account_data.init(
            true,
            *seller_account_info.key,
            *temp_token_account_info.key,
            swap_sol_amount,
            swap_token_amount,
        );

        TokenSaleProgramData::pack(
            token_sale_program_account_data,
            &mut token_sale_program_account_info.try_borrow_mut_data()?,
        )?;

        let (pda, _bump_seed) = Pubkey::find_program_address(&[b"token_sale"], token_sale_program_id);

        let token_program = next_account_info(account_info_iter)?;
        let set_authority_ix = spl_token::instruction::set_authority(
            token_program.key,
            temp_token_account_info.key,
            Some(&pda),
            spl_token::instruction::AuthorityType::AccountOwner,
            seller_account_info.key,
            &[&seller_account_info.key],
        )?;

        msg!("chage tempToken's Authroity : seller -> token_program");
        invoke(
            &set_authority_ix,
            &[
                token_program.clone(),
                temp_token_account_info.clone(),
                seller_account_info.clone(),
            ],
        )?;

        return Ok(());
    }

    //buyer account info 
    //seller account info 
    //temp token account info - For transfer the token to Buyer
    //token sale program account info - For getting data about TokenSaleProgram
    //system program - For transfer SOL
    //buyer token account info - For the buyer to receive the token
    //token program - For transfer the token
    //pda - For signing when send the token from temp token account 

    fn buy_token(accounts: &[AccountInfo], token_sale_program_id: &Pubkey) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let buyer_account_info = next_account_info(account_info_iter)?; 
        if !buyer_account_info.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        let seller_account_info = next_account_info(account_info_iter)?;
        let temp_token_account_info = next_account_info(account_info_iter)?;

        let token_sale_program_account_info = next_account_info(account_info_iter)?;
        let token_sale_program_account_data = TokenSaleProgramData::unpack(&token_sale_program_account_info.try_borrow_data()?)?;

        if *seller_account_info.key != token_sale_program_account_data.seller_pubkey {
            return Err(ProgramError::InvalidAccountData);
        }

        if *temp_token_account_info.key != token_sale_program_account_data.temp_token_account_pubkey{
            return Err(ProgramError::InvalidAccountData);
        }

        msg!("transfer SOL : buy account -> seller account");
        let transfer_sol_to_seller = system_instruction::transfer(
            buyer_account_info.key, 
            seller_account_info.key, 
            1 * LAMPORTS_PER_SOL
        );

        let system_program = next_account_info(account_info_iter)?;
        invoke(
            &transfer_sol_to_seller,
            &[
                buyer_account_info.clone(),
                seller_account_info.clone(),
                system_program.clone()
            ]
        )?;

        msg!("transfer Token : temp token account -> buyer token account");
        let buyer_token_account_info = next_account_info(account_info_iter)?;
        let token_program = next_account_info(account_info_iter)?;
        let (pda, bump_seed) = Pubkey::find_program_address(&[b"token_sale"], token_sale_program_id);

        let transfer_token_to_buyer_ix = spl_token::instruction::transfer(
            token_program.key,
            temp_token_account_info.key,
            buyer_token_account_info.key,
            &pda,
            &[&pda],
            token_sale_program_account_data.swap_token_amount,
        )?;

        let pda = next_account_info(account_info_iter)?;
        invoke_signed(
            &transfer_token_to_buyer_ix,
            &[
                temp_token_account_info.clone(),
                buyer_token_account_info.clone(),
                pda.clone(), 
                token_program.clone(),
            ],
            &[&[&b"token_sale"[..], &[bump_seed]]],
        )?;

        return Ok(());
    }
}
