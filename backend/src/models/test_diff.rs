#[cfg(test)]
mod tests {
    use crate::models::task_attempt::{TaskAttempt, DiffChunkType};

    #[test]
    fn test_line_based_diff() {
        let old_content = "line 1\nline 2\nline 3\n";
        let new_content = "line 1\nmodified line 2\nline 3\n";
        
        let chunks = TaskAttempt::generate_line_based_diff(old_content, new_content);
        
        // Should have: equal, delete, insert, equal
        assert_eq!(chunks.len(), 4);
        
        // First chunk should be equal
        assert_eq!(chunks[0].chunk_type, DiffChunkType::Equal);
        assert_eq!(chunks[0].content, "line 1\n");
        
        // Second chunk should be delete
        assert_eq!(chunks[1].chunk_type, DiffChunkType::Delete);
        assert_eq!(chunks[1].content, "line 2\n");
        
        // Third chunk should be insert
        assert_eq!(chunks[2].chunk_type, DiffChunkType::Insert);
        assert_eq!(chunks[2].content, "modified line 2\n");
        
        // Fourth chunk should be equal
        assert_eq!(chunks[3].chunk_type, DiffChunkType::Equal);
        assert_eq!(chunks[3].content, "line 3\n");
    }
    
    #[test]
    fn test_line_insertion() {
        let old_content = "line 1\nline 3\n";
        let new_content = "line 1\nline 2\nline 3\n";
        
        let chunks = TaskAttempt::generate_line_based_diff(old_content, new_content);
        
        // Should have: equal, insert, equal
        assert_eq!(chunks.len(), 3);
        
        assert_eq!(chunks[0].chunk_type, DiffChunkType::Equal);
        assert_eq!(chunks[0].content, "line 1\n");
        
        assert_eq!(chunks[1].chunk_type, DiffChunkType::Insert);
        assert_eq!(chunks[1].content, "line 2\n");
        
        assert_eq!(chunks[2].chunk_type, DiffChunkType::Equal);
        assert_eq!(chunks[2].content, "line 3\n");
    }
}
