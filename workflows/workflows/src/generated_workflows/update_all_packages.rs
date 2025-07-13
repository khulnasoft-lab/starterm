use starterm_workflows_types::*;
pub fn workflow() -> Workflow {
    Workflow {
        name: r####"Update all Homebrew packages"####.into(),
        command: r####"brew update && brew upgrade"####.into(),
        tags: vec![r####"homebrew"####.into(), r####"system"####.into()].into_iter().collect(),
        description: Some(
            r####"Update Homebrew and upgrade all installed packages to their latest versions."####
                .into(),
        ),
        arguments: vec![].into_iter().collect(),
        source_url: Some(r####"https://docs.brew.sh/Manpage"####.into()),
        author: Some(r####"starterm"####.into()),
        author_url: Some(r####"https://github.com/khulnasoft-lab/starterm"####.into()),
        shells: vec![].into_iter().collect(),
    }
}
