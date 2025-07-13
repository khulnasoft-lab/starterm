use starterm_workflows_types::*;
pub fn workflow() -> Workflow {
    Workflow {
        name: r####"Check if a number is less than another number"####.into(),
        command: r####"[[ {{integer_a}} -lt {{integer_b}} ]]"####.into(),
        tags: vec![r####"shell"####.into()].into_iter().collect(),
        description: Some(
            r####"Returns an exit code of 0 if integer_a is less than or equal to integer_b."####
                .into(),
        ),
        arguments: vec![
            Argument { name: r####"integer_a"####.into(), description: None, default_value: None },
            Argument { name: r####"integer_b"####.into(), description: None, default_value: None },
        ]
        .into_iter()
        .collect(),
        source_url: Some(r####"https://linuxhint.com/bash-test-command/"####.into()),
        author: Some(r####"Sidratul Muntaha"####.into()),
        author_url: Some(r####"https://linuxhint.com/author/lh_sidratul_muntaha/"####.into()),
        shells: vec![Shell::Zsh, Shell::Bash].into_iter().collect(),
    }
}
