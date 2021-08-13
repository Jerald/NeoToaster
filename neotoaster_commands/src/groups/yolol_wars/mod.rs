use serenity::{
    prelude::*,
    framework::standard::{
        macros::{
            command,
            group
        }
    }
};

use cylon_ast::{
    CylonRoot,
    CylonProg,
    CylonLine
};

use yoloxide::{
    environment::{Environment, ContextMap},
};

#[group]
struct YololWars;
pub struct Player {
    name: String,
    program: CylonRoot,
    pub line: usize,
}

impl Player {
    pub fn new(name: String, program: CylonRoot) -> Self {
        Self {
            name,
            program,
            line: 0
        }
    }
}

pub enum NextTurn {
    Player1,
    Player2
}

pub struct Arena {
    memory: Vec<CylonLine>,
    size: usize,
    environment: Environment,

    next_turn: NextTurn,
    player_1: Player,
    player_2: Player
}

impl Arena {    
    pub fn new(size: usize, player_1: Player, player_2: Player) -> Self {
        Self {
            memory: Vec::new(),
            size,
            environment: Environment::new("Arena"),

            next_turn: NextTurn::Player1,
            player_1,
            player_2
        }
    }

    pub fn new_default_for_testing(mut player_1: Player, mut player_2: Player) -> Self {
        player_1.line = 5;
        player_2.line = 30;

        let mut arena = Self::new(50, player_1, player_2);

        let p1start = 5usize;
        let p2start = 30usize;

        arena.memory.get_mut(p1start..).expect("Bad things...")
            .iter_mut()
            .zip(arena.player_1.program.program.lines.iter())
            .for_each(|(mem, line)| {
                *mem = line.clone();
            });

        arena.memory.get_mut(p2start..).expect("Bad things...")
            .iter_mut()
            .zip(arena.player_2.program.program.lines.iter())
            .for_each(|(mem, line)| {
                *mem = line.clone();
            });

        arena
    }
}