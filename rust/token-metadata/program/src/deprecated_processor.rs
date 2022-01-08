use solana_program::msg;

use crate::{
    error::MetadataError,
    state::{
        get_reservation_list, Data, Key, MasterEditionV1, Metadata, Reservation, ReservationListV2,
        EDITION, MAX_MASTER_EDITION_LEN, MAX_RESERVATIONS, MAX_RESERVATION_LIST_SIZE, PREFIX,
        RESERVATION,
    },
    utils::{
        assert_data_valid, assert_derivation, assert_initialized,
        assert_mint_authority_matches_mint, assert_owned_by, assert_rent_exempt, assert_signer,
        assert_supply_invariance, assert_token_program_matches_package,
        assert_update_authority_is_correct, create_or_allocate_account_raw, mint_limited_edition,
        process_create_metadata_accounts_logic, puff_out_data_fields, spl_token_burn,
        spl_token_mint_to, transfer_mint_authority, CreateMetadataAccountsLogicArgs,
        TokenBurnParams, TokenMintToParams,
    },
};
use borsh::BorshSerialize;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    rent::Rent,
    sysvar::Sysvar,
};
use spl_token::state::{Account, Mint};

pub fn process_deprecated_create_metadata_accounts<'a>(
    program_id: &'a Pubkey,
    accounts: &'a [AccountInfo<'a>],
    data: Data,
    allow_direct_creator_writes: bool,
    is_mutable: bool,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let metadata_account_info = next_account_info(account_info_iter)?;
    let mint_info = next_account_info(account_info_iter)?;
    let mint_authority_info = next_account_info(account_info_iter)?;
    let payer_account_info = next_account_info(account_info_iter)?;
    let update_authority_info = next_account_info(account_info_iter)?;
    let system_account_info = next_account_info(account_info_iter)?;
    let rent_info = next_account_info(account_info_iter)?;

    process_create_metadata_accounts_logic(
        &program_id,
        CreateMetadataAccountsLogicArgs {
            metadata_account_info,
            mint_info,
            mint_authority_info,
            payer_account_info,
            update_authority_info,
            system_account_info,
            rent_info,
        },
        data,
        allow_direct_creator_writes,
        is_mutable,
        false,
    )
}

/// Update existing account instruction
pub fn process_deprecated_update_metadata_accounts(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    optional_data: Option<Data>,
    update_authority: Option<Pubkey>,
    primary_sale_happened: Option<bool>,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    let metadata_account_info = next_account_info(account_info_iter)?;
    let update_authority_info = next_account_info(account_info_iter)?;
    let mut metadata = Metadata::from_account_info(metadata_account_info)?;

    assert_owned_by(metadata_account_info, program_id)?;
    assert_update_authority_is_correct(&metadata, update_authority_info)?;

    if let Some(data) = optional_data {
        if metadata.is_mutable {
            assert_data_valid(
                &data,
                update_authority_info.key,
                &metadata,
                false,
                update_authority_info.is_signer,
                true,
            )?;
            metadata.data = data;
        } else {
            return Err(MetadataError::DataIsImmutable.into());
        }
    }

    if let Some(val) = update_authority {
        metadata.update_authority = val;
    }

    if let Some(val) = primary_sale_happened {
        if val {
            metadata.primary_sale_happened = val
        } else {
            return Err(MetadataError::PrimarySaleCanOnlyBeFlippedToTrue.into());
        }
    }

    puff_out_data_fields(&mut metadata);

    metadata.serialize(&mut *metadata_account_info.data.borrow_mut())?;
    Ok(())
}

/// Create master edition
pub fn process_deprecated_create_master_edition(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    max_supply: Option<u64>,
) -> ProgramResult {
    return Err(MetadataError::Deprecated.into());
}

pub fn process_deprecated_mint_new_edition_from_master_edition_via_printing_token<'a>(
    program_id: &'a Pubkey,
    accounts: &'a [AccountInfo<'a>],
) -> ProgramResult {
    return Err(MetadataError::Deprecated.into());
}

pub fn process_deprecated_create_reservation_list(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    return Err(MetadataError::Deprecated.into());
}

pub fn process_deprecated_set_reservation_list(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    reservations: Vec<Reservation>,
    total_reservation_spots: Option<u64>,
    offset: u64,
    total_spot_offset: u64,
) -> ProgramResult {
    return Err(MetadataError::Deprecated.into());
}

pub fn process_deprecated_mint_printing_tokens_via_token(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    supply: u64,
) -> ProgramResult {
    return Err(MetadataError::Deprecated.into());
}

pub fn process_deprecated_mint_printing_tokens(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    supply: u64,
) -> ProgramResult {
    return Err(MetadataError::Deprecated.into());
}
