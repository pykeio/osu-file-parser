use std::{path::PathBuf, str::FromStr};

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use osu_file_parser::osu_file::{
    events::{
        storyboard::{
            cmds::{Command, CommandProperties},
            sprites::{Layer, Object, ObjectType, Origin, Sprite},
        },
        Event,
    },
    Position,
};

pub fn storyboard_cmds_bench(c: &mut Criterion) {
    let fade_str = "F,0,500,1000,0,0.5";
    let move_str = "M,0,500,1000,0,1,2,3";
    let move_x_str = "MX,0,500,1000,0,1";
    let move_y_str = "MY,0,500,1000,0,1";
    let scale_str = "S,0,500,1000,0,0.5";
    let vector_scale_str = "V,0,500,1000,0,0,0.5,0.5";
    let rotate_str = "R,0,500,1000,0,0.5";
    let colour_str = "C,0,500,1000,0,0,0,255,255,255";
    let parameter_str = "P,0,500,1000,H";
    let loop_str = "L,0,10";
    let trigger_str = "T,HitSound,500,1000";

    let mut group = c.benchmark_group("storyboard_cmds");

    group.bench_function("fade_cmd", |b| {
        b.iter(|| {
            Command::from_str(black_box(fade_str)).unwrap();
        })
    });
    group.bench_function("move_cmd", |b| {
        b.iter(|| {
            Command::from_str(black_box(move_str)).unwrap();
        })
    });
    group.bench_function("move_x_cmd", |b| {
        b.iter(|| {
            Command::from_str(black_box(move_x_str)).unwrap();
        })
    });
    group.bench_function("move_y_cmd", |b| {
        b.iter(|| {
            Command::from_str(black_box(move_y_str)).unwrap();
        })
    });
    group.bench_function("scale_cmd", |b| {
        b.iter(|| {
            Command::from_str(black_box(scale_str)).unwrap();
        })
    });
    group.bench_function("vector_scale_cmd", |b| {
        b.iter(|| {
            Command::from_str(black_box(vector_scale_str)).unwrap();
        })
    });
    group.bench_function("rotate_cmd", |b| {
        b.iter(|| {
            Command::from_str(black_box(rotate_str)).unwrap();
        })
    });
    group.bench_function("colour_cmd", |b| {
        b.iter(|| {
            Command::from_str(black_box(colour_str)).unwrap();
        })
    });
    group.bench_function("parameter_cmd", |b| {
        b.iter(|| {
            Command::from_str(black_box(parameter_str)).unwrap();
        })
    });
    group.bench_function("loop_cmd", |b| {
        b.iter(|| {
            Command::from_str(black_box(loop_str)).unwrap();
        })
    });
    group.bench_function("trigger_cmd", |b| {
        b.iter(|| {
            Command::from_str(black_box(trigger_str)).unwrap();
        })
    });
}

pub fn storyboard_loop_cmd_display(c: &mut Criterion) {
    let loop_cmd = |commands| Command {
        start_time: 0,
        properties: CommandProperties::Loop {
            loop_count: 5,
            commands,
        },
    };

    let event = Event::Storyboard(Object {
        layer: Layer::Background,
        origin: Origin::BottomCentre,
        position: Position::default(),
        object_type: ObjectType::Sprite(Sprite {
            filepath: PathBuf::new(),
        }),
        commands: vec![loop_cmd(vec![loop_cmd(vec![loop_cmd(vec![loop_cmd(
            vec![loop_cmd(Vec::new())],
        )])])])],
    });

    c.bench_function("loop_cmd_display", |b| {
        b.iter(|| black_box(&event).to_string())
    });
}

criterion_group!(benches, storyboard_cmds_bench, storyboard_loop_cmd_display);
criterion_main!(benches);
