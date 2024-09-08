# Technical Specifications: img-pipeline

## Overview

This document outlines the specific technologies, libraries, and architectural decisions to implement the **img-pipeline** project. The project will be developed using the **Rust** programming language and will be organized as a modular system using a Rust workspace to manage multiple executables and shared code.

## 1. **Programming Language**

### Rust

The project will be implemented using the **Rust** programming language. Rust is chosen for its performance and support for multi-threaded and concurrent programming, which aligns with the project requirements.

---

## 2. **Project Structure**

The project will use a **Rust Workspace** to organize the multiple executables and shared code modules. This structure allows for efficient code sharing and modularity.

### Workspace Layout

The workspace will be structured as follows:

```
img-pipeline/
├── Cargo.toml           # Workspace definition
├── blurrer/
│   ├── Cargo.toml       # blurrer crate definition
│   └── src/
│       └── main.rs      # Main code for the blurrer executable
├── edger/
│   ├── Cargo.toml       # edger crate definition
│   └── src/
│       └── main.rs      # Main code for the edger executable
├── publisher/
│   ├── Cargo.toml       # publisher crate definition
│   └── src/
│       └── main.rs      # Main code for the publisher executable
└── img_utils/
    ├── Cargo.toml       # Shared utility crate for common functionality
    └── src/
        └── lib.rs       # Common image processing functions, shared memory management, etc.
```

### Crates Breakdown

- **blurrer**: The binary crate responsible for applying the blur filter in a multi-threaded manner.
- **edger**: The binary crate responsible for applying edge detection in a multi-threaded manner.
- **publisher**: The binary crate responsible for orchestrating both the `blurrer` and `edger` processes, managing shared memory, and coordinating their execution.
- **img_utils**: A shared library crate for reusable code, such as image handling, shared memory, and utility functions.

---

## 3. **Crates (Libraries) to Use**

### A. **Image Handling**

- **Crate**: `image`
  - **Purpose**: This crate will be used to load `.bmp` files, manipulate images (e.g., accessing pixel data), and save the processed images back to disk.

### B. **Parallel Processing**

- **Crate**: `rayon`
  - **Purpose**: This crate will enable multi-threaded parallel processing for both the `blurrer` and `edger` executables, allowing tasks to be divided and processed concurrently.

### C. **Shared Memory for Inter-Process Communication (IPC)**

- **Crate**: `memmap2`
  - **Purpose**: This crate will be used to create and manage memory-mapped files, which allow the `publisher`, `blurrer`, and `edger` executables to access the same image data in shared memory.

### D. **Error Handling and Reporting**

- **Crate**: `color-eyre`
  - **Purpose**: This crate will be used to provide enhanced error reporting with colorful, easy-to-read output. It simplifies error handling and debugging by capturing detailed backtraces and presenting errors in a user-friendly format.

---

## 4. **Concurrency Model**

### A. **Internal Multi-Threading (Parallelism)**

Both the `blurrer` and `edger` executables will use multi-threaded parallel processing to handle image filtering. The `rayon` crate will be used to manage threads and distribute the workload.

- **blurrer**: The image will be divided into smaller sections, and each section will be processed in parallel.
- **edger**: Similarly, the image will be split into smaller sections for parallel edge detection.

### B. **Inter-Process Communication (IPC)**

The **publisher** will coordinate the `blurrer` and `edger` processes by using shared memory. This is accomplished via memory-mapped files, allowing both processes to access and modify a common image buffer in memory.

- **Concurrency Model**: The `publisher` launches both processes in parallel and assigns the **top half** of the image to `blurrer` and the **bottom half** to `edger`. Each process works independently, and once both are finished, the `publisher` combines the results and saves the final image.

---

## 5. **Error Handling**

### A. **Image Processing Errors**

- Each executable (`blurrer`, `edger`, and `publisher`) should handle errors such as:
  - Failure to load or write image files.
  - Corrupted or unsupported image formats.

### B. **Process Coordination Errors**

- The `publisher` should handle errors related to process management, such as:
  - Failed process launches.
  - Crashes or timeouts in the `blurrer` or `edger` processes.
  - Shared memory access failures.

### C. **Graceful Termination**

- In case of an error in one process (e.g., `blurrer`), the `publisher` should ensure that the other process (`edger`) is terminated cleanly, and partial results are discarded.

---

## 6. **Documentation**

- **Code Documentation**: All functions and modules should be thoroughly documented using Rust’s built-in documentation system (`cargo doc`).
