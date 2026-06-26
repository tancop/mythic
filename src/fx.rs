use bevy::animation::{AnimatedBy, AnimationClip, AnimationTargetId};
use bevy::ecs::template::{EntityTemplate, FnTemplate, TemplateContext};
use bevy::platform::collections::HashMap;
use bevy::prelude::*;
use bevy::reflect::{GetTypeRegistration, Typed};
use std::fmt::Debug;

/**
Builder for an [AnimationClip] with multiple curves.

Example usage:
```
let clip = ClipBuilder::new()
   .curve(&Name::new("Root"), animate_property!(UiTransform::scale),
       [0.0, 0.05, 0.1], [1.0, 1.15, 1.1].map(Vec2::splat))
   .curve(&Name::new("Text"), animate_property!(UiTransform::rotation),
       [0.0, 0.03, 0.07, 0.1], [0.0, 15.0, -15.0, 0.0].map(Rot2::degrees))
   .build();
```
*/
pub struct ClipBuilder {
    clip: AnimationClip,
}

impl ClipBuilder {
    pub fn new() -> Self {
        Self {
            clip: AnimationClip::default(),
        }
    }

    pub fn curve<P>(
        mut self,
        target_name: &Name,
        property: P,
        times: impl IntoIterator<Item = f32>,
        points: impl IntoIterator<Item = P::Property>,
    ) -> Self
    where
        P: AnimatableProperty + Clone,
        P::Property: Debug + Clone + FromReflect + Typed + GetTypeRegistration,
    {
        let curve = AnimatableCurve::new(
            property,
            AnimatableKeyframeCurve::new(times.into_iter().zip(points))
                .expect("should be able to build valid curve from samples"),
        );

        let target_id = AnimationTargetId::from_name(target_name);
        self.clip.add_curve_to_target(target_id, curve);

        self
    }

    pub fn build(self) -> AnimationClip {
        self.clip
    }
}

/**
Creates an [AnimationClip] with one curve.

Example usage:
```
let clip = single(&Name::new("Root"), animate_property!(UiTransform::scale),
    [0.0, 0.05, 0.1], [1.0, 1.15, 1.1].map(Vec2::splat));
```
*/
pub fn single<P>(
    target_name: &Name,
    property: P,
    times: impl IntoIterator<Item = f32>,
    points: impl IntoIterator<Item = P::Property>,
) -> AnimationClip
where
    P: AnimatableProperty + Clone,
    P::Property: Debug + Clone + FromReflect + Typed + GetTypeRegistration,
{
    let mut clip = AnimationClip::default();

    let curve = AnimatableCurve::new(
        property,
        AnimatableKeyframeCurve::new(times.into_iter().zip(points))
            .expect("should be able to build valid curve from samples"),
    );

    let target_id = AnimationTargetId::from_name(target_name);
    clip.add_curve_to_target(target_id, curve);

    clip
}

/**
A library holding animations by name. You should hold this in a [Resource]
for reuse.

The builder automatically inserts [AnimationGraph] and [AnimationClip] assets to make
everything work.

Example code:
```
fn setup(
    mut res: ResMut<MyAnim>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
    mut clips: ResMut<Assets<AnimationClip>>,
) {
    let clip: AnimationClip = ...;
    let other_clip: AnimationClip = ...;

    let library = Library::new()
        .add("first", clip)
        .add("second", other_clip)
        .build(graphs.as_mut(), clips.as_mut());

    res.library = library;
}
```
*/
#[derive(Default)]
pub struct Library {
    indexes: HashMap<&'static str, AnimationNodeIndex>,
    graph_handle: Handle<AnimationGraph>,
}

impl Library {
    pub fn new() -> LibraryBuilder {
        LibraryBuilder {
            clips: Vec::new(),
            names: Vec::new(),
        }
    }

    pub fn get_index(&self, name: &'static str) -> Option<AnimationNodeIndex> {
        self.indexes.get(name).copied()
    }
}

#[derive(Clone)]
pub struct LibraryBuilder {
    names: Vec<&'static str>,
    clips: Vec<AnimationClip>,
}

impl LibraryBuilder {
    pub fn add(mut self, name: &'static str, clip: AnimationClip) -> Self {
        self.names.push(name);
        self.clips.push(clip);
        self
    }

    pub fn build(
        &self,
        graphs: &mut Assets<AnimationGraph>,
        clips: &mut Assets<AnimationClip>,
    ) -> Result<Library> {
        let mut indexes = HashMap::new();

        let mut graph = AnimationGraph::default();

        for i in 0..self.names.len() {
            let name = self.names[i];
            let clip = self.clips[i].clone();

            let clip_handle = clips.add(clip);

            let idx = graph.add_clip(clip_handle, 1.0, graph.root);
            indexes.insert(name, idx);
        }

        let graph_handle = graphs.add(graph);

        Ok(Library {
            indexes,
            graph_handle,
        })
    }
}

pub fn target(
    name: Name,
) -> FnTemplate<impl Fn(&mut TemplateContext) -> Result<AnimationTargetId> + Clone, AnimationTargetId>
{
    template(move |_| Ok(AnimationTargetId::from_name(&name)))
}

pub trait GetLibrary {
    fn get_library(&self) -> &Library;
}

/**
Marks this entity as animated by `tmpl`.

Example:
```
bsn! {
   #Root
   Node
   player::<MyAnim>()
   target(Name::new("Root"))
   Children [
       Text("hi there")
       animated_by(#Root)
       target(Name::new("Text"))
   ]
}
```
*/
pub fn animated_by(
    tmpl: EntityTemplate,
) -> FnTemplate<impl Fn(&mut TemplateContext) -> Result<AnimatedBy> + Clone, AnimatedBy> {
    template(move |ctx| Ok(AnimatedBy(tmpl.build_template(ctx)?)))
}

/**
Sets up an [AnimationPlayer] to play animations from a library resource.
The resource type must implement [GetLibrary].

Example:
```
bsn! {
   #Root
   Node
   player::<MyAnim>()
   target(Name::new("Root"))
   Children [
       Text("hi there")
       animated_by(#Root)
       target(Name::new("Text"))
   ]
}
```
*/
pub fn player<R: Resource + GetLibrary>()
-> FnTemplate<impl Fn(&mut TemplateContext) -> Result<AnimationPlayer> + Clone, AnimationPlayer> {
    template(move |ctx| {
        let res = ctx.resource::<R>();
        let library = res.get_library();

        ctx.entity
            .insert(AnimationGraphHandle::from(library.graph_handle.clone()));

        ctx.entity.insert(AnimatedBy(ctx.entity.id()));

        Ok(AnimationPlayer::default())
    })
}
