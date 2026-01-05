# Workers Test Runner Architecture

The `auroka_test_runner` implements a **Client-Server Architecture** (similar to Docker or Bazel). This design is required to bridge the gap between the massive parallelism of `cargo nextest` and the heavy, singleton nature of browsers and runtimes.

## Why a Daemon?

Without a central coordinator, running `cargo nextest` would spawn a separate `workerd` or Browser instance for every single test file, leading to:
1.  **Resource Exhaustion:** 16 Chrome instances opening at once.
2.  **Cold Starts:** Paying the 500ms+ startup penalty for every test.

The **Orchestrator Daemon** solves this by acting as a **Resource Multiplexer**. It keeps a pool of "warm" runners ready and routes test requests to them, ensuring that `cargo nextest` remains fast while the heavy lifting is managed efficiently.

## System Architecture

The system is composed of three layers. The **Control Plane** (Server) sits in the middle, mediating between **Clients** (Users) and **Agents** (Workers).

```mermaid
graph LR
  %% Styling
  classDef client fill:#e3f2fd,stroke:#1565c0,stroke-width:2px;
  classDef server fill:#fff3e0,stroke:#e65100,stroke-width:2px;
  classDef driver fill:#f3e5f5,stroke:#7b1fa2,stroke-width:2px;
  classDef runtime fill:#e8f5e9,stroke:#2e7d32,stroke-width:2px;

  CargoTest["cargo test"]:::client
  CargoNextest["cargo nextest"]:::client
  User:::client

  subgraph Clients
    CLI:::client
    WebUI:::client
  end

  CargoTest <==> CLI
  CargoNextest <==> CLI
  User <==> CLI
  User <==> WebUI
  User <==> CargoNextest
  User <==> CargoTest

  CLI <==> Server
  WebUI <==> Server
  
  Server:::server

  subgraph Runtimes
    subgraph VMs
      bun:::runtime
      nodejs:::runtime
      wasmtime:::runtime
      workerd:::runtime
    end

    subgraph Browsers
      chrome:::runtime
      edge:::runtime
      firefox:::runtime
      safari:::runtime
    end
  end

  subgraph Drivers
    subgraph API
      Reqwest:::driver
    end
    subgraph Web
      ChromeDriver["Chrome"]:::driver
      EdgeDriver["Edge"]:::driver
      FirefoxDriver["Firefox"]:::driver
      Playwright["Playwright"]:::driver
      SafariDriver["Safari"]:::driver
    end
  end

  Server <--> bun
  Server <--> chrome
  Server <--> edge
  Server <--> firefox
  Server <--> nodejs
  Server <--> safari
  Server <--> wasmtime
  Server <--> workerd

  Server <--> Reqwest

  Server <--> ChromeDriver
  Server <--> EdgeDriver
  Server <--> FirefoxDriver
  Server <--> Playwright
  Server <--> SafariDriver
```

