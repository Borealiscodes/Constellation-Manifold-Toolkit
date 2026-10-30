pub trait NdH {
    type Point;
    type Loop;

    fn ndh(&self, p: &Self::Point, l: &Self::Loop) -> Self::Point;
}
