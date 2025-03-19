use bumpalo::{collections::Vec, Bump};

enum Command<'a> {
    Add(Vec<'a, Command<'a>>),
    Sub(Vec<'a, Command<'a>>),
    Break,
}

fn main() {
    let arena = Bump::new();
    let mut cmds = Vec::new_in(&arena);
    action(&arena, &mut cmds);
}

fn action<'a>(arena: &'a Bump, cmds: &mut Vec<'a, Command<'a>>) {
    let mut cmd_stack = Vec::new_in(arena);
    action_recursive(arena, cmds, &mut cmd_stack);
}

fn action_recursive<'a>(
    arena: &'a Bump,
    cmds: &mut Vec<'a, Command<'a>>,
    cmd_stack: &mut Vec<'a, Vec<'a, &Command<'a>>>,
) {
    let mut breaks = Vec::new_in(arena);

    cmds.iter().for_each(|cmd| {
        if matches!(cmd, Command::Break) {
            breaks.push(cmd)
        }
    });

    cmd_stack.push(breaks)
}
