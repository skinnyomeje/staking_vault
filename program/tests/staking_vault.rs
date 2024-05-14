use anchor_lang::{
  solana_program::{self},
  system_program, InstructionData, ToAccountMetas
};
use solana_program::instruction::Instruction;
use solana_program_test::{tokio, ProgramTest};
use solana_sdk::{
  pubkey::Pubkey, signature::Keypair, 
  signer::Signer, transaction::Transaction,
};

use solana_program_test::BanksClient;
use anchor_spl::associated_token;
use crate::solana_program::hash::Hash;

use solana_program::program_pack::Pack;
use spl_associated_token_account::instruction::create_associated_token_account;
use anchor_spl::token::{
  spl_token::{
      self,
      instruction::{initialize_mint, mint_to},
  },
  TokenAccount,
};

use spl_token::{instruction,
state::{Mint},
};
use spl_associated_token_account::*;
use std::error::Error;
use solana_program::system_instruction;

#[tokio::test]
async fn test_initialize(){
  let SetUpTest {
    validator,
    vault_account,
  } = SetUpTest::new();

  let (mut banks_client, payer, recent_blockhash) = validator.start().await;
  
  let mint_account = Keypair::new();

  create_mint(
    &mut banks_client,
    &payer,
    &recent_blockhash,
    &spl_token::id(),
    &mint_account,
    9,
    None
  )
  .await
  .unwrap();


  let init_ix = Instruction{
    program_id: staking_vault::ID,
    accounts: staking_vault::accounts::Initialize {
        signer: payer.pubkey(),
        token_vault_account : vault_account,
        mint: mint_account.pubkey(),
        token_program: spl_token::id(),
        system_program: system_program::ID,
    }
    .to_account_metas(None),
    data: staking_vault::instruction::Initialize {}.data(),
  };

  let init_tx = Transaction::new_signed_with_payer(
    &[init_ix],
    Some(&payer.pubkey()),
    &[&payer],
    recent_blockhash,
  );

  banks_client
    .process_transaction(init_tx)
    .await
    .unwrap();
}



#[tokio::test]
async fn test_stake(){
  let SetUpTest {
    validator,
    vault_account,
  } = SetUpTest::new();
 
  let (mut banks_client, payer, recent_blockhash) = validator.start().await;

  let amount = 5;
  let user_token_account_key = &payer;

  let mint_account = Keypair::new();

  create_mint(
    &mut banks_client,
    &payer,
    &recent_blockhash,
    &spl_token::id(),
    &mint_account,
    9,
    Some((user_token_account_key.pubkey(), 10)),
  )
  .await
  .unwrap();


  create_ata(
    &mut banks_client,
    &user_token_account_key,
    &recent_blockhash,
    &spl_token::id(),
    &mint_account.pubkey(),
    &user_token_account_key,
  )
  .await
  .unwrap(); 

  let (stake_info_account_pda, _) = Pubkey::find_program_address(&[b"stake_info", payer.pubkey().as_ref()], &staking_vault::id());
  let (stake_account_pda, _) = Pubkey::find_program_address(&[b"token", payer.pubkey().as_ref()], &staking_vault::id());

  let stake_token_account= associated_token::get_associated_token_address(&stake_account_pda, &mint_account.pubkey());

  let stake_ix = Instruction{
    program_id: staking_vault::ID,
    accounts: staking_vault::accounts::Stake {
      signer: payer.pubkey(),
      stake_info_account: stake_info_account_pda,
      stake_account: stake_token_account,
      user_token_account: user_token_account_key.pubkey(),
      mint: mint_account.pubkey(),
      token_program: spl_token::id(),
      associated_token_program: associated_token::ID,
      system_program: system_program::ID,
    }
    .to_account_metas(None),
    data: staking_vault::instruction::Stake {amount}.data(),
  };

  let stake_tx = Transaction::new_signed_with_payer(
    &[stake_ix],
    Some(&payer.pubkey()),
    &[&payer],
    recent_blockhash,
  );

  banks_client
    .process_transaction(stake_tx)
    .await
    .unwrap();

}

#[tokio::test]
async fn test_destake(){
  let SetUpTest {
    validator,
    vault_account,
  } = SetUpTest::new();
 
  let (mut banks_client, payer, recent_blockhash) = validator.start().await;

  let user_token_account_key = &payer;
  let mint_account = Keypair::new();

  create_mint(
    &mut banks_client,
    &payer,
    &recent_blockhash,
    &spl_token::id(),
    &mint_account,
    9,
    Some((vault_account, 10)),
  )
  .await
  .unwrap();

  create_ata(
    &mut banks_client,
    &user_token_account_key,
    &recent_blockhash,
    &spl_token::id(),
    &mint_account.pubkey(),
    &user_token_account_key,
  )
  .await
  .unwrap();
  

  let (stake_info_account_pda, _) = Pubkey::find_program_address(&[b"stake_info", payer.pubkey().as_ref()], &staking_vault::id());
  let (stake_account_pda, _) = Pubkey::find_program_address(&[b"token", payer.pubkey().as_ref()], &staking_vault::id());


  let destake_ix = Instruction{
    program_id: staking_vault::ID,
    accounts: staking_vault::accounts::DeStake {
      signer: payer.pubkey(),
      stake_info_account: stake_info_account_pda,
      stake_account: stake_account_pda,
      user_token_account: user_token_account.pubkey(),
      token_vault_account: vault_account,
      mint: mint_account.pubkey(),
      token_program: spl_token::id(),
      associated_token_program: associated_token::ID,
      system_program: system_program::ID,
    }
    .to_account_metas(None),
    data: staking_vault::instruction::DeStake {}.data(),
  };

  let destake_tx = Transaction::new_signed_with_payer(
    &[destake_ix],
    Some(&payer.pubkey()),
    &[&payer],
    recent_blockhash,
  );

  banks_client
    .process_transaction(destake_tx)
    .await
    .unwrap();

}

struct SetUpTest{
  pub validator: ProgramTest,
  pub vault_account: Pubkey,
  
}
impl SetUpTest {
  pub fn new() -> SetUpTest{
      let mut validator = ProgramTest::default();
      validator.add_program("staking_vault", staking_vault::ID, None);

      let (vault_account, _) = Pubkey::find_program_address(&[b"vault"], &staking_vault::ID);

      Self {
        validator,
        vault_account,

      
      }
  }
}

pub async fn create_mint(
  banks_client: &mut BanksClient,
  payer: &Keypair,
  recent_blockhash: &Hash,
  program_id: &Pubkey,
  mint_account: &Keypair,
  decimals: u8,
  mint_to: Option<(Pubkey, u64)>,
) -> Result<(), Box<dyn Error>> {
  let mint_rent = solana_sdk::rent::Rent::default().minimum_balance(Mint::LEN);
  let mint_pubkey = mint_account.pubkey();
  let mut ixs = Vec::new();

  let create_ix = system_instruction::create_account(
      &payer.pubkey(),
      &mint_pubkey,
      mint_rent,
      Mint::LEN as u64,
      program_id,
  );
  
  let mint_ix = instruction::initialize_mint(
    program_id,
    &mint_account.pubkey(),
    &payer.pubkey(),
    None,
    decimals,
  )
  .unwrap();

  ixs.push(create_ix);
  ixs.push(mint_ix);

  if let Some((dest, amount)) = mint_to {
    let token_account = associated_token::get_associated_token_address(&dest, &mint_pubkey);
    let create_account_ix =
      spl_associated_token_account::instruction::create_associated_token_account(
        &payer.pubkey(),
        &dest,
        &mint_pubkey,
        &spl_token::id(),
    );

    let mint_to_ix = spl_token::instruction::mint_to(
        &spl_token::id(),
        &mint_pubkey,
        &token_account,
        &payer.pubkey(),
        &[],
        amount,
    )
    .unwrap();

    ixs.push(create_account_ix);
    ixs.push(mint_to_ix);
  }

  let transaction = Transaction::new_signed_with_payer(
      &ixs,
      Some(&payer.pubkey()),
      &[payer, mint_account],
      *recent_blockhash,
  );
  banks_client
      .process_transaction(transaction)
      .await
      .map_err(|e| e.into())
}

pub async fn create_ata(
  banks_client: &mut BanksClient,
  payer: &Keypair,
  recent_blockhash: &Hash,
  token_id: &Pubkey,
  mint: &Pubkey,
  account: &Keypair
) -> Result <(), Box<dyn Error>> {
  let mut ixs = Vec::new();

  let create_account_ix = create_associated_token_account(
    &payer.pubkey(),
    &account.pubkey(),
    &mint,
    token_id,
  );

  ixs.push(create_account_ix);

  let transaction = Transaction::new_signed_with_payer(
    &ixs,
    Some(&payer.pubkey()),
    &[payer, account],
    *recent_blockhash,
  );

  banks_client
    .process_transaction(transaction)
    .await
    .map_err(|e| e.into())
} 

