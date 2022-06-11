use crate::prelude::*;

#[system]
#[read_component(Health)]
#[read_component(Player)]
#[read_component(Item)]
#[read_component(Carried)]
#[read_component(Name)]
#[read_component(Weapon)]
pub fn hud(ecs: &SubWorld) {
    let player_health = <&Health>::query()
        .filter(component::<Player>())
        .iter(ecs)
        .nth(0)
        .unwrap();

    // Player health
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(3);
    draw_batch.print_centered(1, "Explore the Dungeon. Cursor keys to move");
    draw_batch.bar_horizontal(
        Point::zero(),
        SCREEN_WIDTH * 2,
        player_health.current,
        player_health.max,
        ColorPair::new(RED, BLACK),
    );
    draw_batch.print_color_centered(
        0,
        format!(" Health: {} / {}", player_health.current, player_health.max),
        ColorPair::new(WHITE, RED),
    );

    let (player, map_level) = <(Entity, &Player)>::query()
        .iter(ecs)
        .find_map(|(entity, player)| Some((*entity, player.map_level)))
        .unwrap();

    draw_batch.print_color_right(
        Point::new(SCREEN_WIDTH * 2, 1),
        format!("Dungeon Level: {}", map_level + 1),
        ColorPair::new(YELLOW, BLACK),
    );

    let mut weapons: Vec<String> = Vec::new();

    // Player weapons
    let mut y = 3;
    <(&Carried, &Weapon, &Name)>::query()
        .iter(ecs)
        .filter(|(carried, _, _)| carried.0 == player)
        .for_each(|(_, _, name)| {
            draw_batch.print(Point::new(3, y), format!("{}", &name.0));
            weapons.push(name.0.clone());
            y += 1;
        });
    if y > 3 {
        draw_batch.print_color(
            Point::new(3, 2),
            "[ Current Weapon ]",
            ColorPair::new(YELLOW, BLACK),
        );
    }

    // Player inventory
    let mut item_query = <(&Item, &Name, &Carried)>::query();
    let count = item_query
        .iter(ecs)
        .filter(|(_, _, carried)| carried.0 == player)
        .filter(|(_, name, _)| !weapons.contains(&name.0))
        .count() as i32;

    let mut index = 1;

    let mut y = (SCREEN_HEIGHT * 2) - 6 - count;
    item_query
        .iter(ecs)
        .filter(|(_, _, carried)| carried.0 == player)
        .filter(|(_, name, _)| !weapons.contains(&name.0))
        .for_each(|(_, name, _)| {
            draw_batch.print(Point::new(3, y), format!("{}: {}", index, &name.0));
            y += 1;
            index += 1;
        });
    if y > (SCREEN_HEIGHT * 2) - 6 - count {
        draw_batch.print_color(
            Point::new(3, y - index - 1),
            "[ Items carried ]",
            ColorPair::new(YELLOW, BLACK),
        );
    }

    //Drawing everything
    draw_batch.submit(10000).expect("Batch error HUD");
}
