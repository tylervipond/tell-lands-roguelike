use rltk::Point;

pub enum TargetingAction {
  NoAction,
  Exit,
  Selected(Point)
}