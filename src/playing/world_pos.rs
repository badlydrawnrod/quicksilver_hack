use quicksilver::geom::Vector;

pub trait WorldPos {
    fn world_pos(&self) -> Vector;
    fn angle(&self) -> f32;
}
