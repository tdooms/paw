use proc_macro::TokenStream;

use quote::quote;
use syn::{parse_macro_input, AttributeArgs, FnArg, ItemFn, Meta, NestedMeta, Path};

#[proc_macro_attribute]
pub fn primitive(attr: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as AttributeArgs);
    let input = parse_macro_input!(input as ItemFn);

    let name = match args.get(0) {
        Some(NestedMeta::Meta(Meta::Path(Path { segments, .. }))) => {
            segments.first().unwrap().ident.clone()
        }
        _ => panic!(),
    };

    let lower_name = name.to_string().to_lowercase();

    let arg_assign = |arg: &FnArg| match arg {
        FnArg::Typed(pat_type) => {
            let ident = &pat_type.pat;
            quote! { let #ident = self.#ident; }
        }
        _ => panic!(),
    };

    let param_iter = input.sig.inputs.iter().skip(1);
    let params = quote! { #(#param_iter),* };

    let moves = input.sig.inputs.iter().skip(1).map(arg_assign);
    let block = &input.block;

    let expanded = quote! {
        #[derive(Debug, serde::Serialize, serde::Deserialize)]
        #[serde(deny_unknown_fields)]
        pub struct #name {
            #[serde(flatten)]
            attributes: crate::hittables::Attributes,
            material: Box<dyn crate::materials::Material>,
            #params
        }

        #[typetag::serde(name = #lower_name)]
        impl crate::hittables::Hittable for #name {
            fn sdf(&self, sample: nalgebra::Point3<f64>) -> f64 {
                #(#moves);*;
                let inner = |sample: nalgebra::Point3<f64>| {#block};
                self.attributes.apply(sample, inner)
            }

            fn bounds(&self) -> crate::util::Bounds3 {
                crate::util::Bounds3::default()
            }

            fn color(&self, hit: &crate::Hit) -> crate::util::Color3 {
                self.material.color(hit)
            }
        }
    };

    expanded.into()
}
