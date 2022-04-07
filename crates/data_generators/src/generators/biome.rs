use crate::utils::*;
use proc_macro2::{Ident, TokenStream};

pub fn generate() {
    let mut out = TokenStream::new();
    for namespace in std::fs::read_dir(format!("build/worldgen"))
        .expect("There's no worldgen/ folder. Try removing generated/ and re-running generators")
        .flatten()
    {
        let (biome_upper, biome): (Vec<Ident>, Vec<String>) =
            std::fs::read_dir(namespace.path().join("worldgen").join("biome"))
                .unwrap()
                .flatten()
                .filter_map(|file| {
                    let biome = file
                        .file_name()
                        .to_str()
                        .unwrap()
                        .strip_suffix(".json")?
                        .to_owned();
                    Some((format_ident!("{}", biome.to_uppercase()), biome))
                })
                .unzip();
        let namespace = namespace.file_name().to_str().unwrap().to_owned();
        let namespace_ident = format_ident!("{}", namespace);
        out.extend(quote! {
            pub mod #namespace_ident {
                #(pub const #biome_upper: &str = concat!(#namespace, ':', #biome);)*
            }
        })
    }
    output("crates/api/src/core/biomes.rs", out.to_string().as_str())
}
