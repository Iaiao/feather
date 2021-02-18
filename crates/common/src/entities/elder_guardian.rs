use quill_common::entities::ElderGuardian;
use base::EntityKind;
use ecs::EntityBuilder;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(ElderGuardian).add(EntityKind::ElderGuardian);
}
