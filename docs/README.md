# KERN Documentation

This folder contains the source documentation for the KERN language and toolchain. Files are plain Markdown so they can be rendered with any static site generator (MkDocs, Hugo, Docusaurus, etc.).

Structure:

- `overview.md` – high-level introduction and architecture
- `getting_started.md` – installation and quickstart
- `usage_guide.md` – how to write, compile, and run KERN programs
- `language_reference.md` – language syntax and semantics
- `compiler_runtime.md` – compiler pipeline, flags, and runtime internals
- `apis_integration.md` – integration and extension points
- `examples_tutorials.md` – curated examples and tutorials
- `troubleshooting_faq.md` – common errors and fixes
- `appendices.md` – glossary, version history, references

Rendering suggestion (MkDocs):

```bash
pip install mkdocs mkdocs-material
mkdocs new kern-docs
# copy these markdown files into kern-docs/docs/
mkdocs serve
```

Guidance and best practices referenced in these docs follow principles from Write the Docs and Atlassian's documentation recommendations.
