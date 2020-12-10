use specs::Entity;

#[derive(Copy, Clone)]
pub enum Action {
    Attack(Entity),
    MoveTo(usize),
    Chase(usize),
    OpenDoor(usize)
}
