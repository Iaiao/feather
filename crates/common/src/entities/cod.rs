use quill_common::entities::Cod;
use base::EntityKind;
use ecs::EntityBuilder;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(Cod).add(EntityKind::Cod);
}
