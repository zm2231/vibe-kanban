#!/usr/bin/env python3

def test_diff_logic():
    """Test the logic of our line-based diff algorithm"""
    
    # Test case 1: Line modification
    old_content = "line 1\nline 2\nline 3\n"
    new_content = "line 1\nmodified line 2\nline 3\n"
    
    old_lines = old_content.split('\n')[:-1]  # Remove empty last element
    new_lines = new_content.split('\n')[:-1]
    
    print("Test 1 - Line modification:")
    print(f"Old lines: {old_lines}")
    print(f"New lines: {new_lines}")
    
    # Expected chunks: Equal, Delete, Insert, Equal
    expected_chunks = [
        ("Equal", "line 1\n"),
        ("Delete", "line 2\n"),
        ("Insert", "modified line 2\n"),
        ("Equal", "line 3\n")
    ]
    
    print(f"Expected chunks: {expected_chunks}")
    print()
    
    # Test case 2: Line insertion
    old_content = "line 1\nline 3\n"
    new_content = "line 1\nline 2\nline 3\n"
    
    old_lines = old_content.split('\n')[:-1]
    new_lines = new_content.split('\n')[:-1]
    
    print("Test 2 - Line insertion:")
    print(f"Old lines: {old_lines}")
    print(f"New lines: {new_lines}")
    
    # Expected chunks: Equal, Insert, Equal
    expected_chunks = [
        ("Equal", "line 1\n"),
        ("Insert", "line 2\n"),
        ("Equal", "line 3\n")
    ]
    
    print(f"Expected chunks: {expected_chunks}")

if __name__ == "__main__":
    test_diff_logic()
