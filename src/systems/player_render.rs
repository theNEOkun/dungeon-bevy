use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(RenderDual)]
#[read_component(FieldOfView)]
#[read_component(Player)]
pub fn player_render(ecs: &SubWorld, #[resource] camera: &Camera) {
    let player_fov = <&FieldOfView>::query().filter(component::<Player>()).iter(ecs).nth(0).unwrap();

    let mut draw_batch = DrawBatch::new();
    draw_batch.target(2);

    let offset = Point::new(camera.left_x, camera.top_y);

    <(&Player, &Point, &RenderDual)>::query()
        .iter(ecs)
        .filter(|(_, pos, _)| player_fov.visible_tiles.contains(&pos))
        .for_each(|(_, pos, render)| {
            draw_batch.set((*pos - offset) * 2, render.color, render.glyph.0);
            draw_batch.set(Point::new((pos.x * 2), (pos.y * 2 + 1)) - (offset * 2), render.color, render.glyph.1);
        });

    draw_batch.submit(1000).expect("Batch error");
}
