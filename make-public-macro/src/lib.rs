extern crate core;

use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, DataStruct, DeriveInput, FieldsNamed, Visibility};
use syn::Data::Struct;
use syn::Fields::Named;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::Colon;

struct StructField {
    name: Ident,
    ty: Ident
}

impl ToTokens for StructField {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let name = &self.name;
        let t = &self.ty;

        quote!(pub #name: #t).to_tokens(tokens)
    }
}

impl Parse for StructField {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let _vis: Result<Visibility, _> = input.parse();
        let list = Punctuated::<Ident, Colon>::parse_terminated(input).unwrap();

        Ok(StructField {
            name: list.first().unwrap().clone(),
            ty: list.last().unwrap().clone(),
        })
    }
}

#[proc_macro_attribute]
pub fn public(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);
    println!("{:#?}", ast);
    let name = ast.ident;
    let fields = match ast.data {
        Struct(
            DataStruct {
            fields: Named(
                FieldsNamed {
                    ref named, ..
                }), ..
            }
        ) => named,
        _ => unimplemented!("only works for structs with named fields"),
    };

    let builder_fields = fields.iter().map(|f| {
        syn::parse2::<StructField>(f.to_token_stream()).unwrap()
    });

    let public_version = quote! {
        pub struct #name {
            #(#builder_fields,)*
        }
    };
    public_version.into()
}