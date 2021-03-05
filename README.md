# Butler

Rust Build tooling for Dart and Flutter

## Why?

What made me create this tool was that I wanted to be able to have my project
 split into separate modules part of the same bigger module. For example, I
  wanted to split the app into: Application, UI, Domain, Services, etc.
  
However, while having multiple modules inside another, I found that when I
 added, changes or upgraded dependencies, I would have to run flutter pub get
  on several modules one by one. After a while it got tedious.
    
Also, When I wanted to have the test coverage for the whole project this
 again became a problem. Not only I had to run it in each but it would also
  not give me the full coverage for each module aggregated.
   
I then decided to create a tool to do this for me. And since I'm learning
 Rust and want to become more skilled with it, I decided to use it as my
  language to build Butler. 
  
  
# TODO

- [ ] 1 - Needs a review and clean up
- [ ] 2 - Setup a release system
- [ ] 3 - Publish different targets for Window, MacOs and Linux


## Documentation

## Extends Flutter build tasks

### USAGE:
```shell
  butler [SUBCOMMAND]
```

### FLAGS:

 -h, --help       Prints help information 

 -V, --version    Prints version information

### SUBCOMMANDS:

command | description 
--------|------------
**coverage**  |  run coverage on root lib only
**coverall**  |  run tests in all modules and aggregate the coverage
**hello**    |   Check if this is working
**help**     |   Prints this message or the help of the given subcommand(s)
**install**   |  install all dependencies in all modules
**ls**     |    list all dart files
**map-json**   | depends on json_serializer. It will map all annotated classes with @JsonSerialable() inside"/json_mappers/" directories
**run**   |     it will run any dart script in project ./bin folder



