use starterm_workflows::workflows;

fn main() {
    println!("Available Starterm Workflows:\n");
    
    for (i, workflow) in workflows().iter().enumerate() {
        println!("{}. {}", i + 1, workflow.name());
        println!("   Command: {}", workflow.command());
        
        if let Some(desc) = workflow.description() {
            println!("   Description: {}", desc);
        }
        
        if !workflow.tags().is_empty() {
            println!("   Tags: {}", workflow.tags().join(", "));
        }
        
        if !workflow.arguments().is_empty() {
            println!("   Arguments:");
            for arg in workflow.arguments() {
                print!("     - {}: {}", arg.name(), arg.description().as_deref().unwrap_or("No description"));
                if let Some(default) = arg.default_value() {
                    print!(" (default: {})", default);
                }
                println!();
            }
        }
        
        if let Some(author) = workflow.author_name() {
            println!("   Author: {}", author);
        }
        
        println!();
    }
} 