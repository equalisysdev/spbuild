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
- `-p`, `--project-path`: Path to the project config file (If folder passed, defaults to spbuild.json)
- `-o`, `--output-path`: Path to the output folder (where the application is built)

## TODO list
- [ ] Compile a basic application
  - [ ] Compile with MSVC
    - [ ] Single project application
    - [ ] Multi project application
  - [ ] Compile with GCC
    - [ ] Single project application
    - [ ] Multi project application
- [ ] Incremental build support
- [ ] Dependency and package manager (maybe ?)

