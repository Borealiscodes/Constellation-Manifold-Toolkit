pub trait SlackInvariant {
    type Point;

    fn apply_slack(&self, p: &Self::Point, s: &[f64]) -> Self::Point;
}
