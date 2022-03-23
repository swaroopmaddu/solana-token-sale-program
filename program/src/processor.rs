use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::{invoke, invoke_signed},
    program_error::ProgramError,
    program_pack::Pack,
    pubkey::Pubkey,
    system_instruction,
    sysvar::{rent::Rent, Sysvar},
};

use spl_token::state::Account;

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
            TokenSaleInstruction::InitTokenSale { per_token_price, } => {
                msg!("Instruction: init token sale program");
                Self::init_token_sale_program(
                    accounts,
                    per_token_price,
                    token_program_id,
                )
            }
            TokenSaleInstruction::BuyToken { number_of_tokens } => {
                msg!("Instruction: buy token");
                Self::buy_token(accounts, token_program_id, number_of_tokens)
            }

            TokenSaleInstruction::EndTokenSale {} => {
                msg!("Instrcution : end token sale");
                Self::end_token_sale(accounts, token_program_id)
            }
        }
    }

    //seller account info
    //temp token account - TokenAccount isolated by the amount of tokens to be sold
    //token sale program account info - Save the data about token sale
    //rent - To check if the rent fee is exempted
    //token program - To change the owner of temp token account to token sale program
    //per_token_price - Fixed price per each SPL token in LAMPORTS.

    fn init_token_sale_program(
        account_info_list: &[AccountInfo],
        per_token_price: u64,
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
            per_token_price,
        );

        TokenSaleProgramData::pack(
            token_sale_program_account_data,
            &mut token_sale_program_account_info.try_borrow_mut_data()?,
        )?;

        let (pda, _bump_seed) =
            Pubkey::find_program_address(&[b"token_sale"], token_sale_program_id);

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
    // number_of_tokens - Amount of tokens user want to buy

    fn buy_token(accounts: &[AccountInfo], token_sale_program_id: &Pubkey, number_of_tokens:u64) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let buyer_account_info = next_account_info(account_info_iter)?;
        if !buyer_account_info.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        let seller_account_info = next_account_info(account_info_iter)?;
        let temp_token_account_info = next_account_info(account_info_iter)?;

        let token_sale_program_account_info = next_account_info(account_info_iter)?;
        let token_sale_program_account_data =
            TokenSaleProgramData::unpack(&token_sale_program_account_info.try_borrow_data()?)?;

        if *seller_account_info.key != token_sale_program_account_data.seller_pubkey {
            return Err(ProgramError::InvalidAccountData);
        }

        if *temp_token_account_info.key != token_sale_program_account_data.temp_token_account_pubkey
        {
            return Err(ProgramError::InvalidAccountData);
        }

        msg!("Transfer {} SOL : buy account -> seller account",token_sale_program_account_data.per_token_price * number_of_tokens);
        let transfer_sol_to_seller = system_instruction::transfer(
            buyer_account_info.key,
            seller_account_info.key,
            token_sale_program_account_data.per_token_price * number_of_tokens,
        );
        

        let system_program = next_account_info(account_info_iter)?;
        invoke(
            &transfer_sol_to_seller,
            &[
                buyer_account_info.clone(),
                seller_account_info.clone(),
                system_program.clone(),
            ],
        )?;

        msg!("transfer Token : temp token account -> buyer token account");
        let buyer_token_account_info = next_account_info(account_info_iter)?;
        let token_program = next_account_info(account_info_iter)?;
        let (pda, bump_seed) =
            Pubkey::find_program_address(&[b"token_sale"], token_sale_program_id);

        let transfer_token_to_buyer_ix = spl_token::instruction::transfer(
            token_program.key,
            temp_token_account_info.key,
            buyer_token_account_info.key,
            &pda,
            &[&pda],
            number_of_tokens,
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

    //seller_account_info
    //seller_token_account_info - For receive remain token 
    //temp_token_account_info - For retrieve remain token
    //token_program - For transfer the token
    //pda - For signing when send the token from temp token account and close temp token account
    //token sale program account info - To close token sale program
    fn end_token_sale(accounts: &[AccountInfo], token_sale_program_id: &Pubkey) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();

        let seller_account_info = next_account_info(account_info_iter)?;
        let seller_token_account_info = next_account_info(account_info_iter)?;
        let temp_token_account_info = next_account_info(account_info_iter)?;
        let token_program = next_account_info(account_info_iter)?;

        let (pda, _bump_seed) =
            Pubkey::find_program_address(&[b"token_sale"], token_sale_program_id);
        let pda_account_info = next_account_info(account_info_iter)?;

        msg!("transfer Token : temp token account -> seller token account");
        let temp_token_account_info_data = Account::unpack(&temp_token_account_info.data.borrow())?;

        let transfer_token_to_seller_ix = spl_token::instruction::transfer(
            token_program.key,
            temp_token_account_info.key,
            seller_token_account_info.key,
            &pda,
            &[&pda],
            temp_token_account_info_data.amount,
        )?;

        invoke_signed(
            &transfer_token_to_seller_ix,
            &[
                temp_token_account_info.clone(),
                seller_token_account_info.clone(),
                pda_account_info.clone(),
                token_program.clone(),
            ],
            &[&[&b"token_sale"[..], &[_bump_seed]]],
        )?;

        msg!("close account : temp token account -> seller account");
        let close_temp_token_account_ix = spl_token::instruction::close_account(
            token_program.key,
            temp_token_account_info.key,
            seller_account_info.key,
            &pda,
            &[&pda],
        )?;

        invoke_signed(
            &close_temp_token_account_ix,
            &[
                token_program.clone(),
                temp_token_account_info.clone(),
                seller_account_info.clone(),
                pda_account_info.clone(),
            ],
            &[&[&b"token_sale"[..], &[_bump_seed]]],
        )?;

        msg!("close token sale program");
        let token_sale_program_account_info = next_account_info(account_info_iter)?;
        **seller_account_info.try_borrow_mut_lamports()? = seller_account_info
            .lamports()
            .checked_add(token_sale_program_account_info.lamports())
            .ok_or(ProgramError::InsufficientFunds)?;
        **token_sale_program_account_info.try_borrow_mut_lamports()? = 0;
        *token_sale_program_account_info.try_borrow_mut_data()? = &mut [];

        return Ok(());
    }
}
