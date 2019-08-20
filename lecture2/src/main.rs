use mdp::{MDP, ActionOutcome, StateData, ActionData};

random_policy(s: StateData) -> usize {
    
}

fn main() {
    let mut g = MDP::new();
    let facebook = g.add_state("Facebook");
    let class1 = g.add_state("Class1");
    let class2 = g.add_state("Class2");
    let class3 = g.add_state("Class3");
    let sleep = g.add_state("Sleep");
    g.add_action(facebook, vec!(ActionOutcome::new(facebook)), -1.0);
    g.add_action(facebook, vec!(ActionOutcome::new(class1)), 0.0);
    g.add_action(class1, vec!(ActionOutcome::new(facebook)), -1.0);
    g.add_action(class1, vec!(ActionOutcome::new(class2)), -2.0);
    g.add_action(class2, vec!(ActionOutcome::new(sleep)), 0.0);
    g.add_action(class2, vec!(ActionOutcome::new(class3)), -2.0);
    g.add_action(class3, vec!(ActionOutcome::new(sleep)), 10.0);
    g.add_action(class3, vec!(ActionOutcome::new_with_prob(class1, 0.2)), 1.0);
    g.add_action(class3, vec!(ActionOutcome::new_with_prob(class2, 0.4)), 1.0);
    g.add_action(class3, vec!(ActionOutcome::new_with_prob(class3, 0.4)), 1.0);

    g.step(class1, policy)    
}
