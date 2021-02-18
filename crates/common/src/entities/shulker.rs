use quill_common::entities::Shulker;
use base::EntityKind;
use ecs::EntityBuilder;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(Shulker).add(EntityKind::Shulker);
}
