use starterm_workflows_types::*;
pub fn workflow() -> Workflow {
    Workflow {name: r####"Drop all tables and run Laravel's migrations"####.into(),command: r####"php artisan migrate:fresh"####.into(),tags: vec![r####"Laravel"####.into(),r####"Php"####.into()].into_iter().collect(),description: Some(r####"This command will drop all existing tables and run Laravel database migrations after that"####.into()),arguments: vec![].into_iter().collect(),source_url: Some(r####"https://laravel.com/docs/10.x/migrations#running-migrations"####.into()),author: Some(r####"Bert De Swaef"####.into()),author_url: Some(r####"https://github.com/BurtDS"####.into()),shells: vec![].into_iter().collect(),}
}
