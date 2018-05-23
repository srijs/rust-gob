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
        syn::Data::Enum(data) => derive_enum_impl(ident.clone(), data, attr_container, &cx),
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

fn derive_enum_impl(
    ident: syn::Ident,
    data: syn::DataEnum,
    attr_container: attr::Container,
    cx: &Ctxt,
) -> quote::Tokens {
    let len = data.variants.len();

    let mut expanded_type_ids = quote!{};
    for (variant_idx, variant) in data.variants.iter().enumerate() {
        expanded_type_ids.append_all(derive_register_field_types(
            variant_idx,
            variant.fields.iter(),
        ));
    }

    let mut expanded_build_type = quote!{
        serde_schema::types::Type::build()
            .enum_type(stringify!(#ident), #len)
    };

    for (variant_idx, variant) in data.variants.iter().enumerate() {
        let attr_variant = attr::Variant::from_ast(cx, variant);
        match variant.fields {
            syn::Fields::Named(ref fields) => {
                let variant_name = attr_variant.name().serialize_name();
                let fields_len = fields.named.len();
                let mut expanded_inner = quote!{
                    .struct_variant(#variant_name, #fields_len)
                };
                for (field_idx, field) in fields.named.iter().enumerate() {
                    expanded_inner.append_all(derive_field(
                        variant_idx,
                        field_idx,
                        field,
                        &attr_container,
                        cx,
                    ));
                }
                expanded_inner.append_all(quote!{
                    .end()
                });
                expanded_build_type.append_all(expanded_inner);
            }
            syn::Fields::Unnamed(ref fields) => {
                let variant_name = attr_variant.name().serialize_name();
                if fields.unnamed.len() != 1 {
                    panic!("tuple variants are not supported yet")
                }
                let field_type = variant_field_type_variable(variant_idx, 0);
                expanded_build_type.append_all(quote!{
                    .newtype_variant(#variant_name, #field_type)
                });
            }
            syn::Fields::Unit => panic!("unit variants are not supported yet"),
        }
    }

    expanded_build_type.append_all(quote!{
        .end()
    });

    quote!{
        #expanded_type_ids
        ::serde_schema::Schema::register_type(schema, #expanded_build_type)
    }
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

fn variant_field_type_variable(variant_idx: usize, field_idx: usize) -> syn::Ident {
    syn::Ident::from(format!("type_id_{}_{}", variant_idx, field_idx))
}

fn derive_register_field_types<I>(variant_idx: usize, fields: I) -> quote::Tokens
where
    I: IntoIterator,
    I::Item: Borrow<syn::Field>,
{
    let mut expanded = quote!{};
    for (field_idx, field_item) in fields.into_iter().enumerate() {
        let field = field_item.borrow();
        let field_type = &field.ty;
        let type_id_ident = variant_field_type_variable(variant_idx, field_idx);
        expanded.append_all(quote!{
            let #type_id_ident =
                <#field_type as ::serde_schema::SchemaSerialize>::schema_register(schema)?;
        });
    }
    expanded
}

fn derive_field(
    variant_idx: usize,
    field_idx: usize,
    field: &syn::Field,
    attr_container: &attr::Container,
    cx: &Ctxt,
) -> quote::Tokens {
    let attr_field = attr::Field::from_ast(cx, field_idx, field, None, attr_container.default());
    let type_id_ident = variant_field_type_variable(variant_idx, field_idx);
    let field_name = attr_field.name().serialize_name();
    quote!{
        .field(#field_name, #type_id_ident)
    }
}

fn derive_struct_named_fields(
    ident: syn::Ident,
    fields: syn::FieldsNamed,
    attr_container: attr::Container,
    cx: &Ctxt,
) -> quote::Tokens {
    let len = fields.named.len();

    let expanded_type_ids = derive_register_field_types(0, fields.named.iter());

    let mut expanded_build_type = quote!{
        serde_schema::types::Type::build()
            .struct_type(stringify!(#ident), #len)
    };
    for (field_idx, field) in fields.named.iter().enumerate() {
        expanded_build_type.append_all(derive_field(0, field_idx, field, &attr_container, cx));
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
