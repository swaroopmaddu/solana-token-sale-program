use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::{invoke, invoke_signed},
    program_error::ProgramError,
    program_pack::Pack,
    pubkey::Pubkey,
    system_program,
    sysvar::{rent::Rent, Sysvar},
};

use crate::{error::CustomError, instruction::TokenSaleInstruction, state::TokenSaleProgramData};

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
                msg!("swap_sol_amount : {}", swap_sol_amount);
                msg!("swap_token_amount : {}", swap_token_amount);
                Self::init_token_sale_program(
                    accounts,
                    swap_sol_amount,
                    swap_token_amount,
                    token_program_id,
                )
            }
            TokenSaleInstruction::BuyToken { data } => {
                msg!("Instruction: buy token");
                Self::do_what_second_instruction(accounts, data, token_program_id)
            }
        }
    }

    //wallet account -
    //temp token account - sale하고자하는 토큰을 고립시키는 용도
    //escrow program account - escrow program을 init하기 위함
    //rent - rent fee exempt여부를 체크하기 위함
    //token program - temp token의 authority를 escrow program에 넘기기 위함

    fn init_token_sale_program(
        account_info_list: &[AccountInfo],
        swap_sol_amount: u64,
        swap_token_amount: u64,
        token_program_id: &Pubkey,
    ) -> ProgramResult {
        let account_info_iter = &mut account_info_list.iter();

        //유저 월렛
        let wallet_account_info = next_account_info(account_info_iter)?;
        if !wallet_account_info.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        //유저의 토큰 account (렌트 면제 체크 안필요한가?)
        let temp_token_account_info = next_account_info(account_info_iter)?;
        if *temp_token_account_info.owner != spl_token::id() {
            return Err(ProgramError::IncorrectProgramId);
        }

        //escrow 프로그램 초기화를 위한 id
        let token_sale_program_account_info = next_account_info(account_info_iter)?;

        //랜트 면제 체크
        let rent_account_info = next_account_info(account_info_iter)?;
        let rent = &Rent::from_account_info(rent_account_info)?;

        //if not error
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
            *wallet_account_info.key,
            *temp_token_account_info.key,
            swap_sol_amount,
            swap_token_amount,
        );

        TokenSaleProgramData::pack(
            token_sale_program_account_data,
            &mut token_sale_program_account_info.try_borrow_mut_data()?,
        )?;

        let (pda, _bump_seed) = Pubkey::find_program_address(&[b"token_sale"], token_program_id);

        let token_program = next_account_info(account_info_iter)?;
        let set_authority_ix = spl_token::instruction::set_authority(
            token_program.key,
            temp_token_account_info.key,
            Some(&pda),
            spl_token::instruction::AuthorityType::AccountOwner,
            wallet_account_info.key,
            &[&wallet_account_info.key],
        )?;

        msg!("Call set authority (wallet -> pda)");
        invoke(
            &set_authority_ix,
            &[
                token_program.clone(),
                temp_token_account_info.clone(),
                wallet_account_info.clone(),
            ],
        )?;

        return Ok(());
    }

    fn do_what_second_instruction(
        accounts: &[AccountInfo],
        data: u64,
        program_id: &Pubkey,
    ) -> ProgramResult {
        let account_iter = &mut accounts.iter();
        return Ok(());
    }
}
