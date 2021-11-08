# CompilerWebApp

CompilerWebApp is a web server that allows users to study their code online without any burden of environment setup. 

## Major Features
* Customers can input, edit, view the code on the main product’s page
* Customers can choose compilations options
* Customers can submit their code and receive the results of the compilation

## User's guide
### How to run the app
To run the app simply launch it with `cargo run`.
It doesn't require anything specific, but it additionaly allows the admin to set a folder for temporary user source code and compiled binary files. To do this, specify it as command line argument when launching the app: `cargo run "dir_path"`.
This path is stored in the `COMPILATION_TEMP_DIR` environment variable:
- if it already exists when the app is launched - its existing value will be used;
- if the admin specified a value for the variable - it will be used, even if the variable already exists;
- if it doesn't exist - default path will be used: `{workspace_folder}/tempdata`. Same goes for cases when already existing value or admin specified value are not valid paths.

### Endpoints
- `GET: "/"`
Returns `index.html`.

- `POST: "/submit"`
Content-Type: `"application/json"`
body: 
```
{
    "code": "source code",
    "lang": "language",
    "options": "compiler options"
}
```

Returns result of the compilation:
Content-Type: `"application/json"`
body: 
```
{
    "status code": int,
    "stdout": "stdout",
    "stderr": "stderr"
}
```
