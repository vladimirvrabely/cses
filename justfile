set shell := ["/bin/bash", "-c"]

# Show this help
default:
    @just --list

qa-py:
    uv run ruff format
    uv run ruff check --fix
    uv run mypy ./python

run-py problem:
    #!/usr/bin/env bash
    for f in tests/{{problem}}/*.in
        do echo $f; diff -y <(python3 python/{{replace(problem, "-", "_")}}.py < $f) ${f%in}out
    done

build problem:
    cargo fmt
    cargo build --release --bin {{problem}}

run-rs problem:
    #!/usr/bin/env bash
    for f in tests/{{problem}}/*.in
        do echo $f; diff -y <(./target/release/{{problem}} < $f) ${f%in}out
    done

perf-rs problem:
    hyperfine --runs 5 "just run-rs {{problem}}"

perf-py problem:
    hyperfine --runs 5 "just run-py {{problem}}"
