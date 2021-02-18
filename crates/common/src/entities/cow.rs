use quill_common::entities::Cow;
use base::EntityKind;
use ecs::EntityBuilder;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(Cow).add(EntityKind::Cow);
}
