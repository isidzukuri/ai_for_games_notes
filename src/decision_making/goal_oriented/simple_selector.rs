use crate::decision_making::goal_oriented::Action;
use crate::decision_making::goal_oriented::Goal;

pub struct SimpleSelecor;

impl SimpleSelecor {
    pub fn choose_action(_actions: Vec<Action>, _goals: Vec<&Goal>) -> Action {
        // pick goal to achieve
        // let top_goal = Self::top_goal(goals);

        // pick action to satisfy a goal
        // for action in actions.iter() {
        //   action.get_goal_change(top_goal)
        // }
        Action {}
    }

    pub fn top_goal(goals: Vec<&Goal>) -> &Goal {
        goals
            .iter()
            .max_by(|a, b| a.value.partial_cmp(&b.value).unwrap())
            .unwrap()
    }
}
