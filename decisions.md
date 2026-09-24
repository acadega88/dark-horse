# Decisions

Record decisions that shape the project, along with their reasons and any limits. Revisit them when new evidence changes the trade-offs.

## 2026-09-24 — Project name: Dark Horse

Use **Dark Horse** as the working name. The name has personal meaning because the project owner's son loves horses, and the expression appeals to the owner. This is a working project name, not a completed trademark clearance.

## 2026-09-24 — Main language: Rust

Use Rust as the main language. The project owner wants to learn systems programming, and Rust is a suitable language for a performance-conscious application. Do not change this direction without discussing it first.

## 2026-09-24 — Target desktop platforms

Target macOS, Linux, and Windows from the start. Check platform support when selecting each significant dependency; cross-platform intent does not replace testing on each operating system.

## 2026-09-24 — First window library: winit

Use `winit` for the first window prototype. It provides cross-platform window creation and event handling while keeping the prototype small. It does not draw window contents; choose a rendering approach later when needed.

## 2026-09-24 — Development and learning approach

Keep implementation incremental. Explain important concepts and architectural choices before adding them. The project owner prefers to install and run tools through the terminal, with step-by-step instructions.
