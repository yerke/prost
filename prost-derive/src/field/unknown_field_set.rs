use anyhow::bail;
use quote::quote;
use proc_macro2::TokenStream;
use syn::Meta;

#[derive(Clone, Debug)]
pub struct Field;

impl Field {
    pub fn new(attrs: &[Meta]) -> Result<Option<Field>, anyhow::Error> {
        if attrs.len() != 1 {
            bail!("invalid format for unknown_field_set annotation");
        }
        if attrs[0].path().is_ident("unknown_field_set") {
            return Ok(Some(Field));
        }
        Ok(None)
    }

    /// Returns a statement which encodes the field.
    pub fn encode(&self, ident: TokenStream) -> TokenStream {
        quote! {
            #ident.encode(buf);
        }
    }

    /// Returns an expression which evaluates to the result of decoding the field.
    pub fn merge(&self, _ident: TokenStream) -> TokenStream {
        quote! {}
    }

    /// Returns an expression which evaluates to the encoded length of the field.
    pub fn encoded_len(&self, ident: TokenStream) -> TokenStream {
        quote! {
            #ident.encoded_len()
        }
    }

    pub fn clear(&self, ident: TokenStream) -> TokenStream {
        quote!(#ident = ::std::default::Default::default())
    }
}
