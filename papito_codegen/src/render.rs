use heck::SnakeCase;
use quote::Tokens;
use syn::{Ident, Item, ItemImpl, Path, Type, TypePath};
use syn::punctuated::Pair;
use common::{component_of_state, split_path};

pub fn quote(item: Item) -> Tokens {
    match item {
        Item::Impl(item_impl) => {
            impl_render(item_impl)
        }
        _ => {
            panic!("The `#[render]` attribute is only allowed for impl blocks");
        }
    }
}

fn impl_render(item_impl: ItemImpl) -> Tokens {
    let (_, trait_, _) = item_impl.trait_
        .expect("The `#[render]` attribute is only allowed on `papito::prelude::Render` trait impl block");
    let self_ty = *item_impl.self_ty;
    let (comp_ty, assert_mod_ident) = match self_ty.clone() {
        Type::Path(type_path) => {
            component_path_and_assert_ident_of(type_path)
        }
        _ => {
            panic!("Only type paths are allowed to be implemented by `::papito::prelude::Render`");
        }
    };
    let impl_items = item_impl.items;
    quote! {
        mod #assert_mod_ident {
            struct _AssertLifecycle where #self_ty: ::papito::prelude::Lifecycle;
            struct _AssertComponent where #comp_ty: ::papito_dom::Component;
        }

        impl #trait_ for #comp_ty {
            #(#impl_items)*
        }

        impl #trait_ for #self_ty {
            fn render(&self) -> ::papito_dom::prelude::VNode {
                unimplemented!()
            }
        }
    }
}

fn component_path_and_assert_ident_of(type_path: TypePath) -> (Path, Ident) {
    let (mut path, mut last_segment) = split_path(type_path);
    let mod_ident = Ident::from(format!("{}RenderAssertions", &last_segment.ident)
        .to_snake_case());
    last_segment.ident = component_of_state(&last_segment.ident);
    path.segments.push(last_segment);
    (path, mod_ident)
}