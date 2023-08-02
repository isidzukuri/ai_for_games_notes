use crate::decision_making::goal_oriented::Goal;

pub struct Action;

impl Action {
    pub fn get_goal_change(&self, _goal: &Goal) -> f32 {
        3.0
    }
}
