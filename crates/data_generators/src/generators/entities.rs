use convert_case::{Case, Casing};
use proc_macro2::TokenStream;

use crate::utils::*;

pub fn generate() {
    let entities: Vec<EntityInfo> = load_minecraft_json("entities.json").unwrap();
    let mut out = generate_enum!(
        EntityKind,
        entities.par_iter()
            .map(|e| e.name.to_case(Case::UpperCamel))
            .collect::<Vec<_>>(),
        [serde::Serialize, serde::Deserialize, bevy::ecs::component::Component],
        #[serde(try_from = "String", into = "&'static str")]
    );

    out.extend(generate_enum_property!(
        EntityKind,
        "id",
        u32,
        entities
            .iter()
            .map(|e| (e.name.to_case(Case::UpperCamel), {
                let id = e.id;
                quote! { #id }
            }))
            .collect(),
        true
    ));

    out.extend(generate_enum_property!(
        EntityKind,
        "width",
        f64,
        entities
            .iter()
            .map(|e| (e.name.to_case(Case::UpperCamel), {
                let width = e.width;
                quote! { #width }
            }))
            .collect()
    ));

    out.extend(generate_enum_property!(
        EntityKind,
        "height",
        f64,
        entities
            .iter()
            .map(|e| (e.name.to_case(Case::UpperCamel), {
                let height = e.height;
                quote! { #height }
            }))
            .collect()
    ));

    out.extend(generate_enum_property!(
        EntityKind,
        "name",
        &str,
        entities
            .iter()
            .map(|e| (e.name.to_case(Case::UpperCamel), {
                let name = &e.name;
                quote! { #name }
            }))
            .collect(),
        true,
        &'static str
    ));

    out.extend(generate_enum_property!(
        EntityKind,
        "namespaced_id",
        &str,
        entities
            .iter()
            .map(|e| (e.name.to_case(Case::UpperCamel), {
                let namespaced_id = format!("minecraft:{}", e.name);
                quote! { #namespaced_id }
            }))
            .collect(),
        true,
        &'static str
    ));

    out.extend(quote! {
        use std::convert::TryFrom;
        use std::str::FromStr;

        impl TryFrom<String> for EntityKind {
            type Error = &'static str;

            fn try_from(value: String) -> Result<Self, Self::Error> {
                if let Some(kind) = EntityKind::from_name(value.as_str()) {
                    Ok(kind)
                } else {
                    Err("Unknown entity kind")
                }
            }
        }

        impl From<EntityKind> for &'static str {
            fn from(i: EntityKind) -> Self {
                i.name()
            }
        }

        impl FromStr for EntityKind {
            type Err = &'static str;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                if let Some(kind) = EntityKind::from_name(s) {
                    Ok(kind)
                } else {
                    Err("Unknown entity kind")
                }
            }
        }
    });

    output("crates/api/src/core/entity.rs", out.to_string().as_str());

    let mut markers = TokenStream::new();
    for entity in entities.iter() {
        let name = format_ident!("{}", entity.name.to_case(Case::UpperCamel));
        let name_lit = entity.name.clone();
        markers.extend(quote! {
            #[derive(Copy, Clone, Debug, bevy::ecs::component::Component)]
            #[doc = concat!("A marker component for ", #name_lit, " entities.")]
            pub struct #name;
        });
    }
    output(
        "crates/api/src/components/entity_markers.rs",
        markers.to_string().as_str(),
    );

    for entity in entities.iter() {
        let path = &format!("crates/api/src/common/entities/{}.rs", entity.name);
        let file = std::fs::read_to_string(path);
        if file.is_err() || file.unwrap().starts_with(GENERATED_COMMENT) {
            let name = format_ident!("{}", entity.name.to_case(Case::UpperCamel));
            let bundle_name = format_ident!("{}Bundle", entity.name.to_case(Case::UpperCamel));
            output(
                &format!("crates/api/src/common/entities/{}.rs", entity.name),
                quote! {
                    use crate::components::entity_markers::#name;
                    use bevy::ecs::bundle::Bundle;
                    use super::EntityBundle;

                    #[derive(Bundle)]
                    pub struct #bundle_name {
                        pub marker: #name,
                        // TODO replace with LivingEntityBundle if the entity is living,
                        // same with ProjectileBundle and others
                        #[bundle]
                        pub entity: EntityBundle,
                    }
                }
                .to_string()
                .as_str(),
            );
        }
    }

    let name_snake = entities
        .iter()
        .map(|e| format_ident!("{}", e.name))
        .collect::<Vec<_>>();
    output(
        "crates/api/src/common/entities.rs",
        quote! {
            pub use crate::common::generic_bundles::*;

            #(mod #name_snake;)*

            #(pub use #name_snake::*;)*
        }
        .to_string()
        .as_str(),
    );
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct EntityInfo {
    id: u32,
    name: String,
    width: f64,
    height: f64,
}
