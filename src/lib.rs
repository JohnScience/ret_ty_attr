use proc_macro as pm;
use proc_macro2 as pm2;
use quote::ToTokens;

#[proc_macro_attribute]
pub fn ret_ty(meta: pm::TokenStream, input: pm::TokenStream) -> pm::TokenStream {
    fn inner(meta: pm2::TokenStream, input: pm2::TokenStream) -> syn::Result<pm2::TokenStream> {
        let mut input = syn::parse2::<syn::ItemFn>(input)?;
        let ret_ty: syn::ReturnType = if meta.is_empty() {
            syn::ReturnType::Default
        } else {
            syn::parse2::<syn::ReturnType>(meta)?
        };
        let syn::ItemFn {
            sig: syn::Signature { output, .. },
            ..
        } = &mut input;
        *output = ret_ty;
        Ok(input.into_token_stream())
    }
    match inner(meta.into(), input.into()) {
        Ok(output) => output.into(),
        Err(error) => error.into_compile_error().into(),
    }
}
