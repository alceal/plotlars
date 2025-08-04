//! Integration tests for README.md validation and content verification.
//!
//! This test suite validates the README file structure, links, code examples,
//! and ensures documentation consistency and accuracy.

use std::fs;
use std::path::Path;
use regex::Regex;

#[cfg(test)]
mod readme_tests {
    use super::*;

    fn load_readme_content() -> String {
        let readme_path = Path::new("README.md");
        if !readme_path.exists() {
            panic!("README.md file not found");
        }
        fs::read_to_string(readme_path).expect("Failed to read README.md")
    }

    #[test]
    fn test_readme_exists() {
        assert!(Path::new("README.md").exists(), "README.md file should exist");
    }

    #[test]
    fn test_has_title() {
        let content = load_readme_content();
        assert!(
            content.contains("# Plotlars") || content.contains("Plotlars"),
            "README should have 'Plotlars' as title"
        );
    }

    #[test]
    fn test_has_required_sections() {
        let content = load_readme_content();
        let required_sections = vec![
            "## Motivation",
            "## Installation",
            "## Features", 
            "## License",
            "## Acknowledgements",
        ];
        
        for section in required_sections {
            assert!(
                content.contains(section),
                "Section '{}' should be present in README",
                section
            );
        }
    }

    #[test]
    fn test_plots_overview_table_structure() {
        let content = load_readme_content();
        
        // Check for table header
        assert!(
            content.contains("| Plot | Example | Plot | Example | Plot | Example |"),
            "Should have plot overview table header"
        );
        assert!(
            content.contains("|------|:---------:|------|:---------:|------|:---------:|"),
            "Should have table separator"
        );
        
        // Count table rows with plot entries
        let re = Regex::new(r"\| \[.*?\] \| <img.*?> \|").unwrap();
        let matches: Vec<_> = re.find_iter(&content).collect();
        assert!(
            matches.len() >= 5,
            "Should have at least 5 plot types in the table, found {}",
            matches.len()
        );
    }

    #[test]
    fn test_code_blocks_syntax() {
        let content = load_readme_content();
        
        // Check for Rust code blocks
        let rust_re = Regex::new(r"```rust\n(.*?)\n```").unwrap();
        let rust_blocks: Vec<_> = rust_re.captures_iter(&content).collect();
        assert!(
            rust_blocks.len() >= 2,
            "Should have at least 2 Rust code examples, found {}",
            rust_blocks.len()
        );
        
        // Check for bash code blocks
        let bash_re = Regex::new(r"```bash\n(.*?)\n```").unwrap();
        let bash_blocks: Vec<_> = bash_re.captures_iter(&content).collect();
        assert!(
            bash_blocks.len() >= 2,
            "Should have at least 2 bash code examples, found {}",
            bash_blocks.len()
        );
        
        // Verify Rust code blocks contain expected elements
        for capture in rust_blocks {
            let block = capture.get(1).unwrap().as_str();
            if block.contains("main()") {
                assert!(
                    block.contains("use "),
                    "Rust examples should contain use statements"
                );
            }
        }
    }

    #[test]
    fn test_rust_code_syntax_basics() {
        let content = load_readme_content();
        let rust_re = Regex::new(r"```rust\n(.*?)\n```").unwrap();
        
        for (i, capture) in rust_re.captures_iter(&content).enumerate() {
            let block = capture.get(1).unwrap().as_str();
            
            // Check for basic Rust syntax elements
            if block.contains("fn main()") {
                // Check for proper use statements
                if block.to_lowercase().contains("plotlars") {
                    assert!(
                        block.contains("use plotlars::"),
                        "Plotlars example {} should import from plotlars",
                        i + 1
                    );
                }
                if block.to_lowercase().contains("polars") {
                    assert!(
                        block.contains("use polars::prelude::*"),
                        "Polars example {} should import polars prelude",
                        i + 1
                    );
                }
                
                // Check for balanced braces
                let open_braces = block.matches('{').count();
                let close_braces = block.matches('}').count();
                assert_eq!(
                    open_braces, close_braces,
                    "Rust example {} should have balanced braces",
                    i + 1
                );
            }
        }
    }

    #[test]
    fn test_plotlars_api_usage() {
        let content = load_readme_content();
        let rust_re = Regex::new(r"```rust\n(.*?)\n```").unwrap();
        
        let mut plotlars_examples = Vec::new();
        for capture in rust_re.captures_iter(&content) {
            let block = capture.get(1).unwrap().as_str();
            if block.contains("ScatterPlot::builder()") {
                plotlars_examples.push(block);
            }
        }
        
        assert!(
            !plotlars_examples.is_empty(),
            "Should have at least one ScatterPlot example"
        );
        
        for example in plotlars_examples {
            // Check builder pattern usage
            assert!(example.contains(".data("), "ScatterPlot should specify data");
            assert!(example.contains(".x("), "ScatterPlot should specify x axis");
            assert!(example.contains(".y("), "ScatterPlot should specify y axis");
            assert!(example.contains(".build()"), "ScatterPlot should call build()");
            assert!(example.contains(".plot()"), "ScatterPlot should call plot()");
        }
    }

    #[test]
    fn test_installation_commands() {
        let content = load_readme_content();
        let bash_re = Regex::new(r"```bash\n(.*?)\n```").unwrap();
        
        let installation_blocks: Vec<_> = bash_re
            .captures_iter(&content)
            .filter(|cap| cap.get(1).unwrap().as_str().contains("cargo"))
            .collect();
            
        assert!(
            !installation_blocks.is_empty(),
            "Should have cargo installation command"
        );
        
        // Check for correct cargo add command
        let has_cargo_add = installation_blocks
            .iter()
            .any(|cap| cap.get(1).unwrap().as_str().contains("cargo add plotlars"));
        assert!(has_cargo_add, "Should have 'cargo add plotlars' command");
        
        // Check for example running command
        let has_cargo_run = installation_blocks
            .iter()
            .any(|cap| cap.get(1).unwrap().as_str().contains("cargo run --example"));
        assert!(has_cargo_run, "Should have 'cargo run --example' command");
    }

    #[test]
    fn test_plot_types_consistency() {
        let content = load_readme_content();
        
        // Extract plot types from the table
        let plot_re = Regex::new(r"\[([^\]]+)\]\(https://docs\.rs/plotlars/[^)]+\)").unwrap();
        let plot_types: Vec<_> = plot_re
            .captures_iter(&content)
            .map(|cap| cap.get(1).unwrap().as_str().replace(" Plot", "").replace(" ", ""))
            .collect();
        
        // Should have multiple plot types
        assert!(
            plot_types.len() >= 10,
            "Should document at least 10 plot types, found {}",
            plot_types.len()
        );
        
        // Check for expected plot types
        let expected_types = vec!["Array2D", "Bar", "Box", "Scatter", "Line", "Histogram"];
        for expected in expected_types {
            let matching_types: Vec<_> = plot_types
                .iter()
                .filter(|t| t.to_lowercase().contains(&expected.to_lowercase()))
                .collect();
            assert!(
                !matching_types.is_empty(),
                "Should have {} plot type documented",
                expected
            );
        }
    }

    #[test]
    fn test_version_badge_consistency() {
        let content = load_readme_content();
        
        let crates_re = Regex::new(r"https://img\.shields\.io/crates/v/([^\"]+)").unwrap();
        if let Some(cap) = crates_re.captures(&content) {
            assert_eq!(
                cap.get(1).unwrap().as_str(),
                "plotlars",
                "Crates badge should be for plotlars"
            );
        }
        
        let docs_re = Regex::new(r"https://img\.shields\.io/docsrs/([^\"]+)").unwrap();
        if let Some(cap) = docs_re.captures(&content) {
            assert_eq!(
                cap.get(1).unwrap().as_str(),
                "plotlars",
                "Docs badge should be for plotlars"
            );
        }
    }

    #[test]
    fn test_license_consistency() {
        let content = load_readme_content();
        
        // Check for MIT license mention
        assert!(
            content.contains("MIT License"),
            "Should mention MIT License"
        );
        
        // Check for license badge
        assert!(
            content.contains("license-MIT-blue"),
            "Should have MIT license badge"
        );
    }

    #[test]
    fn test_acknowledgements_completeness() {
        let content = load_readme_content();
        
        let ack_re = Regex::new(r"## Acknowledgements\n(.*?)(?=\n##|\z)").unwrap();
        let ack_section = ack_re.captures(&content).expect("Should have acknowledgements section");
        let ack_content = ack_section.get(1).unwrap().as_str();
        
        let expected_acknowledgements = vec!["Polars", "Plotly", "Evcxr", "Rust Community"];
        for ack in expected_acknowledgements {
            assert!(
                ack_content.contains(ack),
                "Should acknowledge {}",
                ack
            );
        }
    }

    #[test]
    fn test_image_references_format() {
        let content = load_readme_content();
        
        // Find all image tags
        let img_re = Regex::new(r"<img[^>]+>").unwrap();
        let img_tags: Vec<_> = img_re.find_iter(&content).collect();
        
        for img_tag in img_tags {
            let tag_str = img_tag.as_str();
            // Should have src attribute
            assert!(
                tag_str.contains("src="),
                "Image tag should have src attribute: {}",
                tag_str
            );
            
            // Should have alt attribute for accessibility (skip imgur images)
            if !tag_str.contains("imgur.com") {
                assert!(
                    tag_str.contains("alt="),
                    "Image tag should have alt attribute: {}",
                    tag_str
                );
            }
        }
    }

    #[test]
    fn test_plot_example_images() {
        let content = load_readme_content();
        
        // Count imgur links (plot examples)
        let imgur_re = Regex::new(r"https://imgur\.com/[^\s\"]+").unwrap();
        let imgur_links: Vec<_> = imgur_re.find_iter(&content).collect();
        
        // Should have multiple plot example images
        assert!(
            imgur_links.len() >= 10,
            "Should have at least 10 plot example images, found {}",
            imgur_links.len()
        );
        
        // Check that images have proper dimensions specified
        let sized_img_re = Regex::new(r#"<img[^>]+width="100"[^>]+height="100"[^>]*>"#).unwrap();
        let sized_images: Vec<_> = sized_img_re.find_iter(&content).collect();
        assert!(
            sized_images.len() >= 10,
            "Plot thumbnails should have specified dimensions, found {}",
            sized_images.len()
        );
    }

    #[test]
    fn test_readme_size_reasonable() {
        let content = load_readme_content();
        let size_kb = content.len() as f64 / 1024.0;
        
        assert!(
            size_kb < 50.0,
            "README should be under 50KB, current size: {:.1}KB",
            size_kb
        );
        assert!(
            size_kb > 5.0,
            "README should be substantial, current size: {:.1}KB",
            size_kb
        );
    }

    #[test]
    fn test_line_length_reasonable() {
        let content = load_readme_content();
        let lines: Vec<&str> = content.split('\n').collect();
        
        let long_lines: Vec<_> = lines
            .iter()
            .filter(|line| line.len() > 120 && !line.trim_start().starts_with('|'))
            .collect();
        
        // Allow some long lines for URLs and code, but not too many
        assert!(
            long_lines.len() < 10,
            "Should have fewer than 10 very long lines, found {}",
            long_lines.len()
        );
    }

    #[test]
    fn test_no_duplicate_content() {
        let content = load_readme_content();
        let lines: Vec<&str> = content.split('\n').collect();
        let non_empty_lines: Vec<&str> = lines
            .iter()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .collect();
        
        // Check for excessive duplication (allowing some for table structure)
        let mut line_counts = std::collections::HashMap::new();
        for line in non_empty_lines {
            if line.len() > 20 {  // Only check substantial lines
                *line_counts.entry(line).or_insert(0) += 1;
            }
        }
        
        let excessive_duplicates: Vec<_> = line_counts
            .iter()
            .filter(|(_, &count)| count > 3)
            .collect();
            
        assert!(
            excessive_duplicates.len() < 3,
            "Should not have excessive duplicate content: {:?}",
            excessive_duplicates.iter().take(3).collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_github_links_format() {
        let content = load_readme_content();
        let github_re = Regex::new(r"https://github\.com/[^\s\)]+").unwrap();
        
        for github_match in github_re.find_iter(&content) {
            let link = github_match.as_str();
            assert!(
                link.starts_with("https://github.com/"),
                "GitHub links should use HTTPS and point to github.com: {}",
                link
            );
            
            let path_parts: Vec<&str> = link
                .strip_prefix("https://github.com/")
                .unwrap()
                .split('/')
                .collect();
            assert!(
                path_parts.len() >= 2,
                "GitHub links should have proper path structure: {}",
                link
            );
        }
    }

    #[test]
    fn test_extract_links_functionality() {
        let content = load_readme_content();
        
        // Match markdown links [text](url)
        let markdown_re = Regex::new(r"\[([^\]]*)\]\(([^)]+)\)").unwrap();
        let markdown_links: Vec<_> = markdown_re.captures_iter(&content).collect();
        
        // Match image src attributes
        let img_re = Regex::new(r#"<img[^>]+src="([^"]+)""#).unwrap();
        let img_links: Vec<_> = img_re.captures_iter(&content).collect();
        
        // Match direct URLs
        let url_re = Regex::new(r"https?://[^\s\)]+").unwrap();
        let direct_links: Vec<_> = url_re.find_iter(&content).collect();
        
        let total_links = markdown_links.len() + img_links.len() + direct_links.len();
        assert!(
            total_links > 0,
            "Should extract at least some links from README, found {}",
            total_links
        );
    }

    #[test]
    fn test_running_examples_section() {
        let content = load_readme_content();
        
        // Check for running examples section
        assert!(
            content.contains("## Running the examples"),
            "Should have 'Running the examples' section"
        );
        
        // Check that it mentions the examples directory
        assert!(
            content.contains("examples/"),
            "Should reference the examples directory"
        );
        
        // Check for --example flag mention
        assert!(
            content.contains("--example"),
            "Should mention the --example flag"
        );
    }

    #[test]
    fn test_jupyter_notebook_section() {
        let content = load_readme_content();
        
        // Check for Jupyter notebooks section
        assert!(
            content.contains("Plotlars in Jupyter Notebooks") || content.contains("Jupyter"),
            "Should mention Jupyter notebook integration"
        );
        
        // Check for evcxr mention
        if content.contains("evcxr") {
            assert!(
                content.contains("https://github.com/evcxr/evcxr"),
                "Should link to evcxr project"
            );
        }
    }

    #[test]
    fn test_feature_descriptions() {
        let content = load_readme_content();
        
        // Check that features are properly described
        let features_section = content.find("## Features");
        assert!(features_section.is_some(), "Should have Features section");
        
        // Look for key feature mentions
        assert!(
            content.contains("Polars") && content.contains("integration"),
            "Should mention Polars integration"
        );
        
        assert!(
            content.contains("plot") && (content.contains("types") || content.contains("Multiple")),
            "Should mention multiple plot types"
        );
    }

    #[test]
    fn test_code_comparison_examples() {
        let content = load_readme_content();
        
        // Should have examples showing before/after or comparison
        assert!(
            content.contains("without Plotlars") || content.contains("using Plotlars"),
            "Should show code comparison examples"
        );
        
        // Count Rust code blocks to ensure there are comparison examples
        let rust_re = Regex::new(r"```rust\n(.*?)\n```").unwrap();
        let rust_blocks: Vec<_> = rust_re.captures_iter(&content).collect();
        
        // Should have substantial examples (at least one longer block)
        let long_examples = rust_blocks
            .iter()
            .filter(|cap| cap.get(1).unwrap().as_str().lines().count() > 10)
            .count();
            
        assert!(
            long_examples >= 1,
            "Should have at least one substantial code example with more than 10 lines"
        );
    }

    #[test]
    fn test_documentation_links() {
        let content = load_readme_content();
        
        // Check for documentation links in the plot table
        let docs_links = Regex::new(r"https://docs\.rs/plotlars/latest/plotlars/")
            .unwrap()
            .find_iter(&content)
            .count();
            
        assert!(
            docs_links >= 5,
            "Should have multiple links to documentation, found {}",
            docs_links
        );
    }

    #[test]
    fn test_badge_formatting() {
        let content = load_readme_content();
        
        // Check for properly formatted badges
        let shield_badges = Regex::new(r"https://img\.shields\.io/")
            .unwrap()
            .find_iter(&content)
            .count();
            
        assert!(
            shield_badges >= 2,
            "Should have at least 2 shield.io badges, found {}",
            shield_badges
        );
        
        // Check that badges are in HTML img tags or markdown format
        let badge_formats = content.contains("<img") || content.contains("![");
        assert!(badge_formats, "Badges should be properly formatted");
    }
}