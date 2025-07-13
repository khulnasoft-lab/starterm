use starterm_workflows_types::*;
pub fn workflow() -> Workflow {
    Workflow {name: r####"Mark node as schedulable"####.into(),command: r####"kubectl uncordon {{node}}"####.into(),tags: vec![r####"kubernetes"####.into()].into_iter().collect(),description: Some(r####"Mark node as schedulable"####.into()),arguments: vec![Argument {name: r####"node"####.into(),description: Some(r####"Node name"####.into()),default_value: None,}].into_iter().collect(),source_url: Some(r####"https://kubernetes.io/docs/reference/generated/kubectl/kubectl-commands#uncordon"####.into()),author: Some(r####"AI"####.into()),author_url: Some(r####"https://kubernetes.io/docs/reference/generated/kubectl/kubectl-commands#uncordon"####.into()),shells: vec![].into_iter().collect(),}
}
