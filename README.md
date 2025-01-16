# RustyGrep

RustyGrep is a file search utility written in Rust. 
It enables you to search for files within a directory (and its subdirectories) or the whole file system.

## Features
- **Regex Support**: Search files using regular expressions for advanced matching.
- **Customizable Starting Directory**: Specify the directory to start the search.
- **Recursive Search**: Searches all subdirectories automatically.
- **Error Handling**: Handles invalid paths, permission issues.

## Usage

Run RustyGrep from the command line with the following syntax:

```bash
rustygrep <regex_pattern> <starting_directory>
```

## Examples

- Search for python files using regex in specific folder

  ```bash
  rustygrep ".*\.py$" PycharmProjects
  ```

- Search for specific file in whole file system 
    ```bash
  rustygrep "presentation.doxc" 
  ```
