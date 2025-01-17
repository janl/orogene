use std::collections::HashMap;

use nassun::Package;
use oro_common::CorgiManifest;
use petgraph::stable_graph::NodeIndex;
use unicase::UniCase;

use crate::Graph;

#[derive(Debug, Clone)]
pub struct Node {
    /// Index of this Node inside its [`Graph`].
    pub(crate) idx: NodeIndex,
    /// Resolved [`Package`] for this Node.
    pub(crate) package: Package,
    /// Resolved [`CorgiManifest`] for this Node.
    pub(crate) manifest: CorgiManifest,
    /// Quick index back to this Node's [`Graph`]'s root Node.
    pub(crate) root: NodeIndex,
    /// Parent, if any, of this Node in the logical filesystem hierarchy.
    pub(crate) parent: Option<NodeIndex>,
    /// Children of this node in the logical filesystem hierarchy. These are
    /// not necessarily dependencies, and this Node's dependencies may not all
    /// be in this HashMap.
    pub(crate) children: HashMap<UniCase<String>, NodeIndex>,
}

impl Node {
    pub(crate) fn new(package: Package, manifest: CorgiManifest) -> Self {
        Self {
            package,
            manifest,
            idx: NodeIndex::new(0),
            root: NodeIndex::new(0),
            parent: None,
            children: HashMap::new(),
        }
    }

    /// This Node's depth in the logical filesystem hierarchy.
    pub(crate) fn depth(&self, graph: &Graph) -> usize {
        let mut depth = 0;
        let mut current = self.parent;
        while let Some(idx) = current {
            depth += 1;
            current = graph.inner[idx].parent;
        }
        depth
    }
}
