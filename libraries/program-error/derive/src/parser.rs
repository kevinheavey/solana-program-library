//! Token parsing

use {
    proc_macro2::Ident,
    syn::{
        parse::{Parse, ParseStream},
        token::Comma,
        LitInt, Token,
    },
};

/// Possible arguments to the `#[spl_program_error]` attribute
pub struct SplProgramErrorArgs {
    /// Whether to hash the error codes using `solana_sha256_hasher::hash`
    /// or to use the default error code assigned by `num_traits`.
    pub hash_error_code_start: Option<u32>,
}

impl Parse for SplProgramErrorArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut hash_error_code_start = None;
        while !input.is_empty() {
            match SplProgramErrorArgParser::parse(input)? {
                SplProgramErrorArgParser::HashErrorCodes { value, .. } => {
                    hash_error_code_start = Some(value.base10_parse::<u32>()?);
                }
            }
        }
        Ok(Self {
            hash_error_code_start,
        })
    }
}

/// Parser for args to the `#[spl_program_error]` attribute
/// ie. `#[spl_program_error(hash_error_code_start = 1275525928)]`
enum SplProgramErrorArgParser {
    HashErrorCodes {
        _equals_sign: Token![=],
        value: LitInt,
        _comma: Option<Comma>,
    },
}

impl Parse for SplProgramErrorArgParser {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ident = input.parse::<Ident>()?;
        match ident.to_string().as_str() {
            "hash_error_code_start" => {
                let _equals_sign = input.parse::<Token![=]>()?;
                let value = input.parse::<LitInt>()?;
                let _comma: Option<Comma> = input.parse().unwrap_or(None);
                Ok(Self::HashErrorCodes {
                    _equals_sign,
                    value,
                    _comma,
                })
            }
            _ => Err(input.error("Expected argument 'hash_error_code_start'")),
        }
    }
}
