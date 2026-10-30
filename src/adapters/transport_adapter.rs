pub trait TransportAdapter {
    type Point;

    fn discrete_transport(&self, p: &Self::Point, lane: u64) -> Self::Point;
}
