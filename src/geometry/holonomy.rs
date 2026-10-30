pub trait Holonomy {
    type Point;
    type Loop;

    fn transport(&self, p: &Self::Point, l: &Self::Loop) -> Self::Point;
}
