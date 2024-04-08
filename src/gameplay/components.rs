use core::time;

use bevy::{prelude::*, time::Stopwatch};

#[derive(Component, Clone)]
pub struct Damage(pub i32);

#[derive(Component, Clone)]
pub struct Speed(pub f32);

#[derive(Component, Clone)]
pub struct ShotSpeed(pub f32);

#[derive(Component, Clone)]
pub struct ReloadStopwatch(pub Stopwatch);

#[derive(Component)]
pub struct ReloadTime(pub time::Duration);

#[derive(Component)]
pub struct Distance(pub f32);

#[derive(Component)]
pub struct Pushed {
    pub distance: Distance,
    pub direction: MyDirection,
    pub speed: Speed,
}

#[derive(Component)]
pub struct Invulnerable {
    pub blink_timer: Timer,
    pub invuln_timer: Timer,
}

#[derive(Component)]
pub struct MyDirection(pub Vec2);

#[derive(Component, Clone, Copy)]
pub enum Shooter {
    Player,
    Enemy,
}

#[derive(Component)]
pub struct HitBlinkTimer {
    pub return_to: Color,
    pub timer: Timer,
}

#[derive(Component, Clone, Copy)]
pub struct Health(pub i32);

#[derive(Component, Clone)]
pub struct RemoveOnReset;

#[derive(Component, Clone)]
pub struct PointWorth(pub u32);

#[derive(Component)]
pub struct Dead;

#[derive(Component)]
pub struct MainCamera;
