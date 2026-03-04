use super::traits::{Tool, ToolArgument, ToolDefinition, ToolResult};
use async_trait::async_trait;
use std::path::Path;

const DEFAULT_MAX_FILE_SIZE: u64 = 10 * 1024 * 1024; // 10MB

#[derive(Debug)]
pub struct FileTool {
    allowed_directory: Option<String>,
    max_file_size: u64,
}

impl FileTool {
    pub fn new() -> Self {
        Self {
            allowed_directory: None,
            max_file_size: DEFAULT_MAX_FILE_SIZE,
        }
    }

    #[allow(dead_code)]
    pub fn with_directory(dir: String) -> Self {
        Self {
            allowed_directory: Some(dir),
            max_file_size: DEFAULT_MAX_FILE_SIZE,
        }
    }

    #[allow(dead_code)]
    pub fn with_max_size(dir: String, max_size: u64) -> Self {
        Self {
            allowed_directory: Some(dir),
            max_file_size: max_size,
        }
    }

    fn is_path_allowed(&self, path: &str) -> bool {
        if let Some(ref allowed) = self.allowed_directory {
            let path = Path::new(path);
            if let Ok(canonical) = path.canonicalize() {
                if let Ok(allowed_canonical) = Path::new(allowed).canonicalize() {
                    return canonical.starts_with(allowed_canonical);
                }
            }
        }
        true
    }
}

impl Default for FileTool {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Tool for FileTool {
    fn name(&self) -> &str {
        "file"
    }

    fn definition(&self) -> ToolDefinition {
        ToolDefinition {
            name: "file".to_string(),
            description: "Read or write files".to_string(),
            arguments: vec![
                ToolArgument {
                    name: "operation".to_string(),
                    arg_type: "string".to_string(),
                    required: true,
                    description: "Operation: read, write, or exists".to_string(),
                },
                ToolArgument {
                    name: "path".to_string(),
                    arg_type: "string".to_string(),
                    required: true,
                    description: "File path".to_string(),
                },
                ToolArgument {
                    name: "content".to_string(),
                    arg_type: "string".to_string(),
                    required: false,
                    description: "Content to write (for write operation)".to_string(),
                },
            ],
        }
    }

    async fn execute(&self, arguments: &str) -> Result<ToolResult, String> {
        let args: serde_json::Value = serde_json::from_str(arguments)
            .map_err(|e| format!("Failed to parse arguments: {}", e))?;

        let operation = args["operation"]
            .as_str()
            .ok_or("Missing 'operation' parameter")?;
        
        let path = args["path"]
            .as_str()
            .ok_or("Missing 'path' parameter")?;

        if !self.is_path_allowed(path) {
            return Ok(ToolResult {
                success: false,
                output: String::new(),
                error: Some("Path not in allowed directory".to_string()),
            });
        }

        match operation {
            "read" => {
                let metadata = tokio::fs::metadata(path).await
                    .map_err(|e| format!("Failed to get file metadata: {}", e))?;
                
                if metadata.len() > self.max_file_size {
                    return Ok(ToolResult {
                        success: false,
                        output: String::new(),
                        error: Some(format!(
                            "File too large: {} bytes (max: {} bytes)",
                            metadata.len(),
                            self.max_file_size
                        )),
                    });
                }

                match tokio::fs::read_to_string(path).await {
                    Ok(content) => Ok(ToolResult {
                        success: true,
                        output: content,
                        error: None,
                    }),
                    Err(e) => Ok(ToolResult {
                        success: false,
                        output: String::new(),
                        error: Some(format!("Failed to read file: {}", e)),
                    }),
                }
            }
            "write" => {
                let content = args["content"]
                    .as_str()
                    .ok_or("Missing 'content' parameter for write operation")?;
                
                if content.len() as u64 > self.max_file_size {
                    return Ok(ToolResult {
                        success: false,
                        output: String::new(),
                        error: Some(format!(
                            "Content too large: {} bytes (max: {} bytes)",
                            content.len(),
                            self.max_file_size
                        )),
                    });
                }
                
                match tokio::fs::write(path, content).await {
                    Ok(_) => Ok(ToolResult {
                        success: true,
                        output: "File written successfully".to_string(),
                        error: None,
                    }),
                    Err(e) => Ok(ToolResult {
                        success: false,
                        output: String::new(),
                        error: Some(format!("Failed to write file: {}", e)),
                    }),
                }
            }
            "exists" => {
                let exists = Path::new(path).exists();
                Ok(ToolResult {
                    success: true,
                    output: exists.to_string(),
                    error: None,
                })
            }
            _ => Ok(ToolResult {
                success: false,
                output: String::new(),
                error: Some(format!("Unknown operation: {}", operation)),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_file_tool_default() {
        let tool = FileTool::new();
        assert_eq!(tool.name(), "file");
    }

    #[test]
    fn test_file_tool_definition() {
        let tool = FileTool::new();
        let def = tool.definition();
        assert_eq!(def.name, "file");
        assert_eq!(def.arguments.len(), 3);
    }

    #[tokio::test]
    async fn test_file_write_and_read() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");
        let file_path_str = file_path.to_string_lossy().to_string();
        
        let tool = FileTool::new();
        
        let write_result = tool.execute(&serde_json::json!({
            "operation": "write",
            "path": file_path_str,
            "content": "Hello, World!"
        }).to_string()).await;
        assert!(write_result.unwrap().success);
        
        let read_result = tool.execute(&serde_json::json!({
            "operation": "read",
            "path": file_path_str
        }).to_string()).await;
        let result = read_result.unwrap();
        assert!(result.success);
        assert_eq!(result.output, "Hello, World!");
    }

    #[tokio::test]
    async fn test_file_exists() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");
        let file_path_str = file_path.to_string_lossy().to_string();
        
        std::fs::write(&file_path, "test").unwrap();
        
        let tool = FileTool::new();
        let result = tool.execute(&serde_json::json!({
            "operation": "exists",
            "path": file_path_str
        }).to_string()).await.unwrap();
        
        assert!(result.success);
        assert_eq!(result.output, "true");
    }

    #[tokio::test]
    async fn test_file_read_nonexistent() {
        let tool = FileTool::new();
        let result = tool.execute(r#"{"operation": "read", "path": "/nonexistent/file.txt"}"#).await.unwrap();
        assert!(!result.success);
    }
}
