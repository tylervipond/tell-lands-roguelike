use specs::Entity;

#[derive(Copy, Clone)]
pub enum Action {
    Attack(Entity),
    MoveTo((i32, i32)),
    Chase((i32, i32)),
    OpenDoor((i32, i32))
}
