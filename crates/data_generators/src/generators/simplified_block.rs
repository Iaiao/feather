use convert_case::{Case, Casing};
use regex::Regex;

use crate::utils::*;
use indexmap::map::IndexMap;

pub fn generate() {
    let simplified_blocks: SimplifiedBlocks = load_feather_json("simplified_block.json").unwrap();
    let blocks: Vec<IdAndName> = load_minecraft_json("blocks.json").unwrap();

    let mut out = quote! { use crate::BlockKind; };

    let mappings = blocks
        .into_iter()
        .map(|block| {
            (
                block.name.to_case(Case::UpperCamel),
                simplified_blocks
                    .regexes
                    .iter()
                    .filter(|(_, regexp)| Regex::new(regexp).unwrap().is_match(&block.name))
                    .map(|(kind, _)| kind.to_case(Case::UpperCamel))
                    .next()
                    .unwrap_or_else(|| block.name.to_case(Case::UpperCamel)),
            )
        })
        .collect::<IndexMap<_, _>>();

    let mut variants = mappings.values().collect::<Vec<_>>();
    variants.sort();
    variants.dedup();
    out.extend(generate_enum!(SimplifiedBlockKind, variants));
    out.extend(generate_enum_property!(
        BlockKind,
        "simplified_kind",
        SimplifiedBlockKind,
        mappings
            .into_iter()
            .map(|(key, value)| (key, {
                let kind = format_ident!("{}", value);
                quote! { SimplifiedBlockKind::#kind }
            }))
            .collect(),
    ));

    output(
        "crates/api/src/blocks/simplified_block.rs",
        out.to_string().as_str(),
    );
}

#[derive(Deserialize)]
struct SimplifiedBlocks {
    regexes: IndexMap<String, String>,
}
