# Project Instructions: img-pipeline

## Overview

The **img-pipeline** project is designed to process bitmap (`.bmp`) images using a series of parallelized image processing filters. The program consists of three executables:

1. **blurrer**: Applies a blur filter to the image.
2. **edger**: Applies an edge detection filter to the image.
3. **publisher**: Coordinates the application of both filters by dividing the image into two halves (top and bottom) and applying one filter to each half concurrently.

Each executable is a standalone process that can operate independently or in coordination with the `publisher` to process images in parallel for optimal efficiency.

## Purpose

The purpose of the img-pipeline project is to:

- Allow the application of **image filters** (blur and edge detection) in a **parallelized** fashion to improve performance.
- Use **inter-process communication (IPC)** to enable concurrent processing of an image’s top and bottom halves when run via the `publisher`.
- Ensure that both `blurrer` and `edger` executables are fully independent, while being capable of being orchestrated by the `publisher` for optimized parallel processing.

---

## Functional Requirements

### 1. **Executable Programs**

The project consists of three executable programs:

#### A. **blurrer**

- **Functionality**:
  - Applies a **blur filter** to the **entire image**.
  - Must support **parallel processing** to enhance performance when applied to the full image.
- **Execution Context**:
  - When run as a standalone process, it reads a `.bmp` file from disk, processes it by applying a blur filter, and writes the result back to disk.
  - When coordinated by the `publisher`, it applies the blur filter to the **top half** of the image provided by the shared memory mechanism.

#### B. **edger**

- **Functionality**:
  - Applies an **edge detection filter** to the **entire image**.
  - Must support **parallel processing** to enhance performance when applied to the full image.
- **Execution Context**:
  - When run as a standalone process, it reads a `.bmp` file from disk, processes it by applying an edge detection filter, and writes the result back to disk.
  - When coordinated by the `publisher`, it applies the edge detection filter to the **bottom half** of the image provided by the shared memory mechanism.

#### C. **publisher**

- **Functionality**:
  - Loads a `.bmp` image and divides it into two halves: the **top half** for the `blurrer` and the **bottom half** for the `edger`.
  - It coordinates the parallel execution of the `blurrer` and `edger` processes, each working on their respective halves of the image.
  - After both processes have completed, it combines the filtered halves and saves the result as a new `.bmp` file.

### 2. **Concurrency and Parallelism Requirements**

- **Independent Execution**:

  - Both `blurrer` and `edger` should be able to process images independently, each applying its respective filter across the entire image.
  - Each executable must support **internal parallel processing** (multi-threading or other parallel mechanisms) to efficiently process large images.

- **Parallel Execution under Publisher**:
  - The `publisher` will execute the `blurrer` and `edger` processes in **parallel**, with each process working independently on its assigned half of the image.
  - The **order of execution** does not matter. The processes should not block each other, and the final image should be assembled only after both processes have completed their tasks.

### 3. **Error Handling**

- Each executable should handle errors appropriately:
  - If an image file cannot be read or written, the respective process must terminate with a descriptive error message.
  - If there are issues with dividing or processing the image (e.g., corrupted image data), the process must fail gracefully and clean up any intermediate outputs.
  - The `publisher` should ensure that any failure in one of the processes (either `blurrer` or `edger`) is detected and handled appropriately, ensuring partial results are discarded, and the system can recover.

---

## Non-Functional Requirements

### 1. **Performance**

- The system must be designed to process large `.bmp` images efficiently.
- Both the `blurrer` and `edger` processes must leverage parallel processing to ensure high performance when working with high-resolution images.
- The `publisher` should ensure that the `blurrer` and `edger` executables run concurrently, minimizing the overall time required to apply the filters and generate the final image.

### 2. **Scalability**

- The system should be able to scale with increasing image sizes, taking advantage of available hardware resources (e.g., multi-core CPUs) to maintain performance.

### 3. **Reliability**

- The system must ensure reliable processing of images, even under high load or when dealing with large images.
- The `publisher` must guarantee that both `blurrer` and `edger` complete successfully before saving the final image to disk.

### 4. **Maintainability**

- The system should be modular, allowing for future extension (e.g., adding more filters) without significant refactoring.
- The implementation should be clean and well-documented to facilitate future maintenance.

---

## Deliverables

The following deliverables are expected:

- **blurrer** executable: Fully parallelized blur processing program.
- **edger** executable: Fully parallelized edge detection program.
- **publisher** executable: Coordinates the application of both filters using a concurrent execution model.
- **Shared Code Module**: A module or set of modules that handle common functionality, such as loading and saving `.bmp` images, dividing the image for processing, and coordinating IPC.
