extern crate proc_macro;
#[macro_use]
extern crate quote;
extern crate serde_derive_internals;
extern crate syn;

use std::borrow::Borrow;

use serde_derive_internals::{attr, Ctxt};
use syn::DeriveInput;

#[proc_macro_derive(SchemaSerialize)]
pub fn derive_schema_serialize(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input: DeriveInput = syn::parse(input).unwrap();

    let cx = Ctxt::new();
    let attr_container = attr::Container::from_ast(&cx, &input);

    let ident = input.ident;
    let generics = input.generics;

    let inner_impl = match input.data {
        syn::Data::Struct(data) => derive_struct_impl(ident.clone(), data, attr_container, &cx),
        syn::Data::Enum(_) => panic!("enums are not supported yet"),
        syn::Data::Union(_) => panic!("unions are not supported yet"),
    };

    let expanded = quote!{
        impl #generics ::serde_schema::SchemaSerialize for #ident #generics {
            fn schema_register<S>(schema: &mut S) -> Result<S::TypeId, S::Error>
                where S: serde_schema::Schema
            {
                #inner_impl
            }
        }
    };

    cx.check().unwrap();

    expanded.into()
}

fn derive_struct_impl(
    ident: syn::Ident,
    data: syn::DataStruct,
    attr_container: attr::Container,
    cx: &Ctxt,
) -> quote::Tokens {
    match data.fields {
        syn::Fields::Named(fields) => derive_struct_named_fields(ident, fields, attr_container, cx),
        syn::Fields::Unnamed(_fields) => panic!("tuple structs are not supported yet"),
        syn::Fields::Unit => panic!("unit structs are not supported yet"),
    }
}

fn field_type_variable(field_ident: &syn::Ident) -> syn::Ident {
    syn::Ident::from(format!("type_id_{}", field_ident))
}

fn derive_register_field_types<I>(fields: I) -> quote::Tokens
where
    I: IntoIterator,
    I::Item: Borrow<syn::Field>,
{
    let mut expanded = quote!{};
    for field_item in fields {
        let field = field_item.borrow();
        let field_ident = field.ident.as_ref().unwrap();
        let field_type = &field.ty;
        let type_id_ident = field_type_variable(field_ident);
        expanded.append_all(quote!{
            let #type_id_ident =
                <#field_type as ::serde_schema::SchemaSerialize>::schema_register(schema)?;
        });
    }
    expanded
}

fn derive_struct_named_fields(
    ident: syn::Ident,
    fields: syn::FieldsNamed,
    attr_container: attr::Container,
    cx: &Ctxt,
) -> quote::Tokens {
    let len = fields.named.len();

    let expanded_type_ids = derive_register_field_types(fields.named.iter());

    let mut expanded_build_type = quote!{
        serde_schema::types::Type::build()
            .struct_type(stringify!(#ident), #len)
    };
    for (idx, field) in fields.named.iter().enumerate() {
        let attr_field = attr::Field::from_ast(cx, idx, field, None, attr_container.default());
        let field_ident = field.ident.as_ref().unwrap();
        let type_id_ident = field_type_variable(field_ident);
        let field_name = attr_field.name().serialize_name();
        expanded_build_type.append_all(quote!{
            .field(#field_name, #type_id_ident)
        });
    }
    expanded_build_type.append_all(quote!{
        .end()
    });

    quote!{
        #expanded_type_ids
        ::serde_schema::Schema::register_type(schema, #expanded_build_type)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
