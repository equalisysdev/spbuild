# Single Player Build System (spbuild)
> The build system for singleplayer devs

**WARNING: PROJECT IS NOT READY FOR USE YET. Please note that some of these options don't work yet**

## About
The goal of this project is to help little teams and solo devs configuring a multiplatform dev environment in a [WORA](https://en.wikipedia.org/wiki/Write_once,_run_anywhere) fashion.
To use, you only need to run
```bash
spbuild build [OPTIONS]
```
Here are some available options:
- `-s`, `--solution-path`: Path to the project config file (If folder passed, defaults to spbuild.json)
- `-o`, `--output-path`: Path to the output folder (where the application is built). Defaults to project-path/output

## Naming
- Project : A single target for the compiler (executable, library, etc.)
- Solution : A collection of projects (like Visual Studio solutions)
- Project's output directory : The directory where the compiled files of a single project are stored
- Project source : The directory where the project's source code is located.. usually right next to spbuild.json
- Solution root : The directory where spbuild.json is located

### Project configuration file options
- `name` : Name of the project. Can be any string
- `version`: Version of the project. Can be any string
- `project_type`: Type of the project. Can be one of the following:
  - `Executable`: A standalone application
  - `StaticLib`: A static library that can be linked to other projects
  - `DynamicLib`: A dynamic library (like DLLs on Windows)
- `target_archs`: List of target architectures. Can be any of the following:
  - `X64`: 64-bit architecture
  - `x86`: 32-bit architecture
  - `ARM64`: ARM 64-bit architecture
  - `ARM`: ARM 32-bit architecture
- `target_platforms`: List of target platforms. Can be any of the following:
  - `windows`: Microsoft Windows
  - `linux`: Linux-based operating systems
  - `macos`: Apple's MacOS
- `path`: The path to the project folder (relative to the solution root)
- `dependencies`: List of other projects that this project depends on (by name). If a dependency is not found in
    the solution, spbuild will look for it in the package manager (not implemented yet)

## TODO list
- [ ] Compile a basic solution
  - [ ] Compile with MSVC
    - [ ] Single project solution
    - [ ] Multi project solution
  - [ ] Compile with GCC
    - [x] Single project solution
    - [ ] Link
    - [ ] Multi project solution
    - [ ] Link
- [ ] Incremental build support
- [ ] Dependency and package manager (definitely)

## Message to future me
1. Write tests
2. Test point number 1
3. Use absolute paths everywhere
4. Document everything
