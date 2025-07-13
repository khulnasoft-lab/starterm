use starterm_workflows_types::*;
pub fn workflow() -> Workflow {
    Workflow {
        name: r####"Git commit with message"####.into(),
        command: r####"git add . && git commit -m '{{message}}'"####.into(),
        tags: vec![r####"git"####.into(), r####"version-control"####.into()].into_iter().collect(),
        description: Some(r####"Stage all changes and commit with a custom message"####.into()),
        arguments: vec![Argument {
            name: r####"message"####.into(),
            description: Some(r####"Commit message"####.into()),
            default_value: Some(r####"Update"####.into()),
        }]
        .into_iter()
        .collect(),
        source_url: Some(r####"https://git-scm.com/docs/git-commit"####.into()),
        author: Some(r####"starterm"####.into()),
        author_url: Some(r####"https://github.com/khulnasoft-lab/starterm"####.into()),
        shells: vec![].into_iter().collect(),
    }
}
