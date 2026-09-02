use std::marker::PhantomData;

use bevy::{
    ecs::{system::IntoObserverSystem, template::TemplateContext},
    input::keyboard::Key,
    input_focus::InputFocus,
    prelude::*,
    scene::{ResolveContext, ResolveSceneError, ResolvedScene},
};

#[derive(EntityEvent)]
pub struct EnterPressed(Entity);

pub fn on_keyboard_input(focus: Res<InputFocus>, input: Res<ButtonInput<Key>>, mut cmd: Commands) {
    if input.just_pressed(Key::Enter)
        && let Some(entity) = focus.get()
    {
        cmd.entity(entity).trigger(EnterPressed);
    }
}

/// A [`Template`] / [`Scene`] that will create an [`Observer`] of a given [`Event`] on the current [`World`].
///
/// [`Observer`]: bevy_ecs::observer::Observer
pub struct ListenTemplate<I, E, B, M>(pub I, pub PhantomData<fn() -> (E, B, M)>);

impl<I: IntoObserverSystem<E, B, M> + Clone, E: Event, B: Bundle, M: 'static> Template
    for ListenTemplate<I, E, B, M>
{
    type Output = ();

    fn build_template(&self, context: &mut TemplateContext) -> Result<Self::Output> {
        context.entity.world_scope(|scope| {
            scope.add_observer(self.0.clone());
        });
        Ok(())
    }

    fn clone_template(&self) -> Self {
        Self(self.0.clone(), PhantomData)
    }
}

impl<I: IntoObserverSystem<E, B, M> + Clone + Send + Sync, E: Event, B: Bundle, M: 'static> Scene
    for ListenTemplate<I, E, B, M>
{
    fn resolve(
        self,
        _context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        scene.push_bundle_template(ListenTemplate(self.0, PhantomData));
        Ok(())
    }
}

/// Returns a [`ListenTemplate`] that will create an [`Observer`] of a given [`Event`] on the current [`World`].
///
/// [`Observer`]: bevy_ecs::observer::Observer
pub fn listen<I: IntoObserverSystem<E, B, M>, E: Event, B: Bundle, M: 'static>(
    observer: I,
) -> ListenTemplate<I, E, B, M> {
    ListenTemplate(observer, PhantomData)
}
