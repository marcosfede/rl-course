type StateIndex = usize;

#[derive(Debug)]
pub struct StateData {
    label: &'static str,
    first_outgoing_edge: Option<ActionIndex>,
}

type ActionIndex = usize;

#[derive(Debug)]
pub struct ActionOutcome {
    pub state: StateIndex,
    pub probability: f64,
}

impl ActionOutcome {
    pub fn new(state: StateIndex) -> ActionOutcome {
        ActionOutcome {
            state,
            probability: 1.0,
        }
    }
    pub fn new_with_prob(state: StateIndex, probability: f64) -> ActionOutcome {
        ActionOutcome { state, probability }
    }
}

#[derive(Debug)]
pub struct ActionData {
    pub outcomes: Vec<ActionOutcome>,
    next_outgoing_edge: Option<ActionIndex>,
    pub reward: f64,
}

#[derive(Debug)]
pub struct MDP {
    nodes: Vec<StateData>,
    edges: Vec<ActionData>,
}

pub struct ActionsIterator<'graph> {
    mdp: &'graph MDP,
    current_edge_index: Option<ActionIndex>,
}

impl<'graph> Iterator for ActionsIterator<'graph> {
    type Item = &'graph ActionData;
    fn next(&mut self) -> Option<&'graph ActionData> {
        match self.current_edge_index {
            None => None,
            Some(edge_num) => {
                let edge = &self.mdp.edges[edge_num];
                self.current_edge_index = edge.next_outgoing_edge;
                Some(edge)
            }
        }
    }
}

impl MDP {
    pub fn new() -> MDP {
        MDP {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }

    pub fn add_state(&mut self, label: &'static str) -> StateIndex {
        let index = self.nodes.len();
        self.nodes.push(StateData {
            label,
            first_outgoing_edge: None,
        });
        index
    }

    pub fn add_action(&mut self, source: StateIndex, outcomes: Vec<ActionOutcome>, reward: f64) {
        let edge_index = self.edges.len();
        let node_data = &mut self.nodes[source];
        self.edges.push(ActionData {
            outcomes: outcomes,
            next_outgoing_edge: node_data.first_outgoing_edge,
            reward,
        });
        node_data.first_outgoing_edge = Some(edge_index);
    }

    pub fn actions(&self, source: StateIndex) -> ActionsIterator {
        let first_outgoing_edge = self.nodes[source].first_outgoing_edge;
        ActionsIterator {
            mdp: self,
            current_edge_index: first_outgoing_edge,
        }
    }
}

pub trait Policy {
    fn choose_action(s: StateData) -> ActionData;
}