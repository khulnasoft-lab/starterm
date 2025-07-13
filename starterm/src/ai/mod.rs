//! The AI Core for Starterm, providing intelligent, context-aware assistance.
//!
//! This module integrates various AI capabilities, from natural language
//! understanding to workflow generation, voice commands, and visual analysis.

pub mod agent;
pub mod context;
pub mod core;
pub mod learning;
pub mod multimodal;
pub mod reasoning;
pub mod suggestions;
pub mod vision;
pub mod voice;
pub mod workflow_gen;

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_ai_core_creation() {
        let ai_core = core::AiCore::new();
        assert!(ai_core.get_suggestions(&context::AiContext {
            current_input: "",
            scrollback: vec![],
            cwd: "/tmp",
            shell_type: "bash",
            last_exit_code: None,
        }).is_empty());
    }

    #[tokio::test]
    async fn test_workflow_generation() {
        let context = context::AiContext {
            current_input: "",
            scrollback: vec![],
            cwd: "/tmp",
            shell_type: "bash",
            last_exit_code: None,
        };

        let result = workflow_gen::generate_workflow_from_prompt(
            "list files and count them",
            &context
        ).await;

        assert!(result.is_ok());
        let workflow = result.unwrap();
        assert_eq!(workflow.workflow.name(), "Generated: List and Count");
        assert!(workflow.workflow.command().contains("ls -l"));
    }
} 