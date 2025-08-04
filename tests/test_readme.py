"""
Unit tests for README.md validation and content verification.

This test suite validates the README file structure, links, code examples,
and ensures documentation consistency and accuracy.

Testing Framework: pytest (as no existing framework was found)
"""

import re
import pytest
from pathlib import Path


class TestReadmeStructure:
    """Test the structural elements of the README file."""
    
    @pytest.fixture
    def readme_content(self):
        """Load README content for testing."""
        readme_path = Path("README.md")
        if not readme_path.exists():
            pytest.skip("README.md not found")
        return readme_path.read_text(encoding='utf-8')
    
    def test_readme_exists(self):
        """Test that README.md file exists."""
        assert Path("README.md").exists(), "README.md file should exist"
    
    def test_has_title(self, readme_content):
        """Test that README has a proper title."""
        lines = readme_content.split('\n')
        title_found = False
        for line in lines[:10]:  # Check first 10 lines
            if line.strip() == "# Plotlars" or "Plotlars" in line:
                title_found = True
                break
        assert title_found, "README should have 'Plotlars' as title"
    
    def test_has_required_sections(self, readme_content):
        """Test that README contains all required sections."""
        required_sections = [
            "Motivation",
            "Installation", 
            "Features",
            "License",
            "Acknowledgements"
        ]
        
        for section in required_sections:
            assert f"## {section}" in readme_content, f"Section '{section}' should be present"
    
    def test_plots_overview_table_structure(self, readme_content):
        """Test that the plots overview table is properly formatted."""
        # Check for table header
        assert "| Plot | Example | Plot | Example | Plot | Example |" in readme_content
        assert "|------|:---------:|------|:---------:|------|:---------:|" in readme_content
        
        # Count table rows (should have multiple plot entries)
        table_rows = re.findall(r'\| \[.*?\] \| <img.*?> \|', readme_content)
        assert len(table_rows) >= 5, "Should have at least 5 plot types in the table"
    
    def test_code_blocks_syntax(self, readme_content):
        """Test that code blocks have proper syntax highlighting."""
        rust_blocks = re.findall(r'```rust\n(.*?)\n```', readme_content, re.DOTALL)
        bash_blocks = re.findall(r'```bash\n(.*?)\n```', readme_content, re.DOTALL)
        
        assert len(rust_blocks) >= 2, "Should have at least 2 Rust code examples"
        assert len(bash_blocks) >= 2, "Should have at least 2 bash code examples"
        
        # Verify Rust code blocks contain expected elements
        for block in rust_blocks:
            if "main()" in block:
                assert "use " in block, "Rust examples should contain use statements"


class TestReadmeCodeExamples:
    """Test the code examples in the README for syntax and completeness."""
    
    @pytest.fixture
    def readme_content(self):
        """Load README content for testing."""
        return Path("README.md").read_text(encoding='utf-8')
    
    def test_rust_code_syntax_basics(self, readme_content):
        """Test basic Rust syntax in code examples."""
        rust_blocks = re.findall(r'```rust\n(.*?)\n```', readme_content, re.DOTALL)
        
        for i, block in enumerate(rust_blocks):
            # Check for basic Rust syntax elements
            assert 'fn main()' in block, f"Rust example {i+1} should have main function"
            
            # Check for proper use statements
            if 'plotlars' in block.lower():
                assert 'use plotlars::' in block, f"Plotlars example {i+1} should import from plotlars"
            if 'polars' in block.lower():
                assert 'use polars::prelude::*' in block, f"Polars example {i+1} should import polars prelude"
            
            # Check for proper semicolons and braces
            assert block.count('{') == block.count('}'), f"Rust example {i+1} should have balanced braces"
    
    def test_plotlars_api_usage(self, readme_content):
        """Test that Plotlars API is used correctly in examples."""
        rust_blocks = re.findall(r'```rust\n(.*?)\n```', readme_content, re.DOTALL)
        
        plotlars_examples = [block for block in rust_blocks if 'ScatterPlot::builder()' in block]
        assert len(plotlars_examples) >= 1, "Should have at least one ScatterPlot example"
        
        for example in plotlars_examples:
            # Check builder pattern usage
            assert '.data(' in example, "ScatterPlot should specify data"
            assert '.x(' in example, "ScatterPlot should specify x axis"
            assert '.y(' in example, "ScatterPlot should specify y axis"
            assert '.build()' in example, "ScatterPlot should call build()"
            assert '.plot()' in example, "ScatterPlot should call plot()"
    
    def test_installation_commands(self, readme_content):
        """Test that installation commands are correct."""
        bash_blocks = re.findall(r'```bash\n(.*?)\n```', readme_content, re.DOTALL)
        
        installation_blocks = [block for block in bash_blocks if 'cargo' in block]
        assert len(installation_blocks) >= 1, "Should have cargo installation command"
        
        # Check for correct cargo add command
        has_cargo_add = any('cargo add plotlars' in block for block in installation_blocks)
        assert has_cargo_add, "Should have 'cargo add plotlars' command"
        
        # Check for example running command
        has_cargo_run = any('cargo run --example' in block for block in installation_blocks)
        assert has_cargo_run, "Should have 'cargo run --example' command"


class TestReadmeConsistency:
    """Test for consistency and accuracy across the README."""
    
    @pytest.fixture
    def readme_content(self):
        """Load README content for testing."""
        return Path("README.md").read_text(encoding='utf-8')
    
    def test_plot_types_consistency(self, readme_content):
        """Test that plot types mentioned are consistent throughout."""
        # Extract plot types from the table
        plot_links = re.findall(r'\[([^\]]+)\]\(https://docs\.rs/plotlars/[^)]+\)', readme_content)
        plot_types = [link.replace(' Plot', '').replace(' ', '') for link in plot_links]
        
        # Should have multiple plot types
        assert len(plot_types) >= 10, "Should document at least 10 plot types"
        
        # Check for expected plot types
        expected_types = ['Array2D', 'Bar', 'Box', 'Scatter', 'Line', 'Histogram']
        for expected in expected_types:
            matching_types = [t for t in plot_types if expected.lower() in t.lower()]
            assert len(matching_types) > 0, f"Should have {expected} plot type documented"
    
    def test_version_badge_consistency(self, readme_content):
        """Test that version badges point to the correct package."""
        crates_badge = re.search(r'https://img\.shields\.io/crates/v/([^"]+)', readme_content)
        docs_badge = re.search(r'https://img\.shields\.io/docsrs/([^"]+)', readme_content)
        
        if crates_badge:
            assert crates_badge.group(1) == 'plotlars', "Crates badge should be for plotlars"
        if docs_badge:
            assert docs_badge.group(1) == 'plotlars', "Docs badge should be for plotlars"
    
    def test_license_consistency(self, readme_content):
        """Test that license information is consistent."""
        # Check for MIT license mention
        assert 'MIT License' in readme_content, "Should mention MIT License"
        
        # Check for license badge
        license_badge = re.search(r'license-MIT-blue', readme_content)
        assert license_badge, "Should have MIT license badge"


class TestReadmeImages:
    """Test image references and accessibility."""
    
    @pytest.fixture
    def readme_content(self):
        """Load README content for testing."""
        return Path("README.md").read_text(encoding='utf-8')
    
    def test_image_references_format(self, readme_content):
        """Test that image references are properly formatted."""
        # Find all image tags
        img_tags = re.findall(r'<img[^>]+>', readme_content)
        
        for img_tag in img_tags:
            # Should have src attribute
            assert 'src=' in img_tag, f"Image tag should have src attribute: {img_tag}"
    
    def test_plot_example_images(self, readme_content):
        """Test that plot example images are present."""
        # Count imgur links (plot examples)
        imgur_links = re.findall(r'https://imgur\.com/[^\s"]+', readme_content)
        
        # Should have multiple plot example images
        assert len(imgur_links) >= 10, "Should have at least 10 plot example images"
        
        # Check that images have proper dimensions specified
        sized_images = re.findall(r'<img[^>]+width="100"[^>]+height="100"[^>]*>', readme_content)
        assert len(sized_images) >= 10, "Plot thumbnails should have specified dimensions"


if __name__ == "__main__":
    pytest.main([__file__, "-v"])