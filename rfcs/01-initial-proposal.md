# Feature Name: `bevy_fog_of_war`

## Summary

This RFC plans to create an easy-to-use plugin that will immediately add fog of war to a game.

## Motivation

Fog of war is a feature that many games need yet there is often little information about how to actually design and implement fog of war.

## User-facing explanation

To use, you can add either `FogOfWarPlugin` or `FogOfWarPlugin2d` to your app. Additionally you can configure if visited spaces remain "remembered", meaning you can see static objects beneath the fog of war, once an area has been visited. By default all objects with a Transform and an Aabb are handled as static. Mark them with the `FogOfWarDynamicObject` to have them not revealed, if they are currently not in sight of a revealer.
Add `FogOfWarRevealer` (or `FogOfWarRevealer2d`) component to an entity that should uncover fog of war. Additionally you can specify the shape of the revealer. All primitive bevy offers are supported.

```rust
commands.spawn((
    Name::from("MainCharacter"),
    SpriteBundle::default(),
    FogOfWarRevealer {
        shape: Cone::default(),
        ..default()
    },
));
```

The revealing shape will be aligned with the forward vector of an object and rotate with it, so you don't have to specify anything than an initial, optional offset.

## Implementation strategy

Fog of war implementation should be fast, reliable and usable in virtually every situation a user can think of, that includes on different planes (xy, xz, yz) and both with static as well as dynamic cameras in translation, rotation and scale/size of orthographic or perspective projection. A user should also be able to decide if they want the fog of war to be rendered in front or behind a UI. Although it almost never makes sense to render fog of war in front of a UI, it should be up to the user.

For a fast and usable implementation the best approach would be to draw two textures, one for unrevealed areas and one for revealed areas that aren't currently revealed. Use a shader to overlay these textures during the rendering phase. If the fog of war should be volumetric, a mesh can be extruded from the textures and filled with volumetric fog.

Onto these textures, the revealer will draw the specified shapes.

The chosen strategy of implementation is to start small and concrete with a proof of concept and abstract and generalize from there.

## Drawbacks

The main risk implementing fog of war is, that fog of war itself is something that cannot be generalized and the needs of individual game developers or their games cannot be foreseen. They might find a solid starting point in the fog of war crate but ultimately have to reimplement everything themselves.

## Rationale and alternatives

- Why is this design the best in the space of possible designs?
- What other designs have been considered and what is the rationale for not choosing them?
- What objections immediately spring to mind? How have you addressed them?
- What is the impact of not doing this?

## Prior art

While there is no fog of war crate for bevy yet, fog of war has existed in games for over 25 years.
There are extensive papers, video essays and tutorials on how to implement and use them and what benefits and problems they bring.

* https://medium.com/@travnick/fog-of-war-282c8335a355
* https://www.youtube.com/watch?v=vUYZiQ3C4hU
* https://andrewhungblog.wordpress.com/2018/06/23/implementing-fog-of-war-in-unity/
* https://github.com/ufna/VaFogOfWar
* https://www.youtube.com/live/mJROd-NVHHQ

## Unresolved questions

At this point all major questions should be resolved.
